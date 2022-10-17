#[cfg(test)]
mod test;

use std::{
  error::Error,
  fmt::Display,
};

use crate::{
  game::{
    GameError,
    Unseen,
  },
  rng::{
    IndexedPermutation,
    Konadare192PxPlusPlus,
    KSINK,
  },
};

const DEFAULT_ELEMENT_CHECKSUM: u64 = 2636128771936786712;
const THRESHOLD_MAX: u32 = 1_000_000_000;

pub const INITIAL_LIVES_AMOUNT: usize = 3;

pub type IncorrectCommits = [Option<usize>; INITIAL_LIVES_AMOUNT];

pub struct SeenThreshold(u32);

#[derive(Debug)]
pub enum SeenThresholdError
{
  ValueOutOfRange,
}

impl Display for SeenThresholdError
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    writeln!(
      f,
      "{:?} - seen threshold ratio must be in range [0.0, 1.0]",
      self
    )
  }
}

impl Error for SeenThresholdError {}

impl TryFrom<f64> for SeenThreshold
{
  type Error = SeenThresholdError;

  fn try_from(value: f64) -> Result<Self, Self::Error>
  {
    if value < 0.0 || value > 1.0 {
      Err(SeenThresholdError::ValueOutOfRange)
    } else {
      Ok(SeenThreshold((THRESHOLD_MAX as f64 * value) as u32))
    }
  }
}

#[derive(Clone, Debug)]
pub struct Game<T>
{
  seed: u64,
  unseen: Unseen<T>,
  seen: Vec<T>,
  current: Option<T>,
  previuos: Option<T>,
  incorrect_commits: IncorrectCommits,
  rng: Konadare192PxPlusPlus,
  seen_threshold: u32,
  count: usize,
  element_checksum: u64,
}

impl<T> Game<T>
{
  /// Create a new game.
  pub fn new(seed: u64, seen_threshold: SeenThreshold, unseen: Vec<T>) -> Game<T>
  {
    Game {
      seed,
      unseen: Unseen::new(unseen),
      seen: Vec::new(),
      current: None,
      previuos: None,
      incorrect_commits: [None; 3],
      rng: Konadare192PxPlusPlus::from_seed(seed),
      seen_threshold: seen_threshold.0,
      count: 0,
      element_checksum: DEFAULT_ELEMENT_CHECKSUM,
    }
  }

  /// Reset `self`. Given the same input it will now reproduce its output.
  pub fn reset(&mut self)
  {
    self.unseen.reset();
    self.seen = Vec::new();
    self.current = None;
    self.previuos = None;
    for x in self.incorrect_commits.iter_mut() {
      x.take();
    }
    self.rng = Konadare192PxPlusPlus::from_seed(self.seed);
    self.count = 0;
    self.element_checksum = DEFAULT_ELEMENT_CHECKSUM;
  }

  /// Returns how many lives the game has left.
  pub fn lives(&self) -> usize
  {
    self
      .incorrect_commits
      .iter()
      .map(|x| x.map_or(1, |_| 0))
      .sum()
  }

  /// Returns the score, i.e the amount of correct commits.
  pub fn score(&self) -> usize
  {
    self.count - (INITIAL_LIVES_AMOUNT - self.lives())
  }

  /// Indicies of incorrect commits.
  pub fn incorrect_commits(&self) -> IncorrectCommits
  {
    self.incorrect_commits
  }

  /// Seed used in the game.
  pub fn seed(&self) -> u64
  {
    self.seed
  }

  /// True if three wrong commits has been made.
  pub fn finished(&self) -> bool
  {
    self.incorrect_commits[2].is_some()
  }

  /// Throws `GameError::GameOver` if `Game::finished` is `true`.
  fn game_over(&self) -> Result<(), GameError>
  {
    if self.finished() {
      Err(GameError::GameOver)
    } else {
      Ok(())
    }
  }

  /// The checksum of the generated elements.
  pub fn element_checksum(&self) -> u64
  {
    self.element_checksum
  }
}

impl<T> Game<T>
where
  T: Clone + PartialEq + AsRef<[u8]>,
{
  /// Generates the next value.
  pub fn next(&mut self) -> Result<&T, GameError>
  {
    self.game_over()?;

    // Early exit if there is already a value in `current`.
    self
      .current
      .as_ref()
      .map_or(Ok(()), |_| Err(GameError::NextCalledWithUncommitedResult))?;

    // With that checked it is now possible to generate the next value.

    if self.seen.len() < 2
      || self.rng.next_with_upper_bound(THRESHOLD_MAX as u32) > self.seen_threshold
    {
      self.next_unseen()
    } else {
      self.next_seen()
    }
  }

  fn next_unseen(&mut self) -> Result<&T, GameError>
  {
    let x = self
      .unseen
      .poll(&mut self.rng)
      .map(|x| x.clone())
      .ok_or(GameError::UnseenEmpty)?;

    self.element_checksum = KSINK::hash(self.element_checksum, x.as_ref());
    self.current = Some(x);

    Ok(self.current.as_ref().unwrap())
  }

  fn next_seen(&mut self) -> Result<&T, GameError>
  {
    loop {
      let i = self.rng.next_with_upper_bound(self.seen.len() as u32) as usize;

      // Prevent next generated value from being equal to the previous.
      if self.previuos.as_ref().map_or(true, |x| x != &self.seen[i]) {
        self.current = self.seen.get(i).map(|x| x.clone());
        break;
      }
    }
    self.element_checksum = KSINK::hash(
      self.element_checksum,
      self.current.as_ref().unwrap().as_ref(),
    );
    Ok(self.current.as_ref().unwrap())
  }

  fn push_incorrect_commit(&mut self, x: usize) -> Option<&usize>
  {
    for y in self.incorrect_commits.iter_mut() {
      if y.is_none() {
        y.replace(x);
        return y.as_ref();
      }
    }
    None
  }

  /// Commit the current result as unseen. Returns a `bool` indicatign if the element was unseen.
  pub fn commit_unseen(&mut self) -> Result<bool, GameError>
  {
    self.commit(false)
  }

  /// Commit the current result as seen. Returns a `bool` indicatign if the element was seen.
  pub fn commit_seen(&mut self) -> Result<bool, GameError>
  {
    self.commit(true)
  }

  fn commit(&mut self, seen: bool) -> Result<bool, GameError>
  {
    self.game_over()?;

    if let Some(x) = self.current.take() {
      let r = if !self.seen.contains(&x) {
        self.seen.push(x.clone());
        true
      } else {
        false
      } ^ seen;

      self.previuos.replace(x);
      if !r {
        self.push_incorrect_commit(self.count);
      }
      self.count += 1;
      Ok(r)
    } else {
      Err(GameError::EmptyCommit)
    }
  }
}
