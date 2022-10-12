use std::fmt::Display;

use crate::{
  game::{
    GameError,
    Unseen,
  },
  rng::Konadare192PxPlusPlus,
};

const THRESHOLD_MAX: u32 = 1_000_000_000;

pub const INITIAL_LIVES_AMOUNT: usize = 3;

pub struct SeenThreshold(u32);

#[derive(Debug)]
pub struct SeenThresholdValueOutOfRange;

impl Display for SeenThresholdValueOutOfRange
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

impl TryFrom<f64> for SeenThreshold
{
  type Error = SeenThresholdValueOutOfRange;

  fn try_from(value: f64) -> Result<Self, Self::Error>
  {
    if value < 0.0 || value > 1.0 {
      Err(SeenThresholdValueOutOfRange)
    } else {
      Ok(SeenThreshold((THRESHOLD_MAX as f64 * value) as u32))
    }
  }
}

#[derive(Clone, Debug)]
pub struct Game<T: 'static>
{
  seed: u64,
  unseen: Unseen<T>,
  seen: Vec<T>,
  current: Option<T>,
  previuos: Option<T>,
  incorrect_commits: [Option<usize>; INITIAL_LIVES_AMOUNT],
  rng: Konadare192PxPlusPlus,
  seen_threshold: u32,
  count: usize,
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
  pub fn incorrect_commits(&self) -> [Option<usize>; 3]
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
}

impl<T> Game<T>
where
  T: Clone + PartialEq,
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
    self.current = Some(
      self
        .unseen
        .poll(&mut self.rng)
        .map(|x| x.clone())
        .ok_or(GameError::UnseenEmpty)?,
    );
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