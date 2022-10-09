use crate::{
  game::GameError,
  rng::Konadare192PxPlusPlus,
};

const THRESHOLD_MAX: f64 = 1e9;

#[derive(Debug)]
pub struct Game<T>
{
  #[allow(dead_code)]
  seed: u64,

  unseen: Vec<Option<T>>,
  seen: Vec<T>,
  current: Option<T>,
  previuos: Option<T>,
  strike: [Option<u32>; 3],
  rng: Konadare192PxPlusPlus,
  seen_threshold: u32,
  count: u32,
}

impl<T> Game<T>
where
  T: Clone + PartialEq,
{
  /// Create a new game.
  pub fn new(seed: u64, seen_ratio: f64, unseen: impl Iterator<Item = T>) -> Game<T>
  {
    Game {
      seed,
      unseen: unseen.map(|x| Some(x)).collect(),
      seen: Vec::new(),
      current: None,
      previuos: None,
      strike: [None; 3],
      rng: Konadare192PxPlusPlus::from_seed(seed),
      seen_threshold: (seen_ratio * THRESHOLD_MAX).round() as u32,
      count: 0,
    }
  }

  /// True if three wrong commits has been made.
  pub fn finished(&self) -> bool
  {
    self.strike[2].is_some()
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
    match self.unseen.len() {
      0 => Err(GameError::UnseenEmpty),
      n => {
        let n = n.try_into().unwrap();

        // Generate a random index in `unseen`.
        let i = self.rng.next_with_upper_bound(n);

        if i == n - 1 {
          // If `i` is the last index, just pop it.
          self.current = self.unseen.pop().unwrap();
        } else {
          // If `i` is not the last index, set current to `unseen[i]` and then replace index `i`
          // with the last element in `unseen`.
          self.current = self.unseen[i as usize].take();
          self.unseen[i as usize] = self.unseen.pop().unwrap();
        }
        Ok(self.current.as_ref().unwrap())
      }
    }
  }

  fn next_seen(&mut self) -> Result<&T, GameError>
  {
    loop {
      let i = self.rng.next_with_upper_bound(self.seen.len() as u32) as usize;

      // Prevent next generated value from being equal to the previous.
      if self.previuos.as_ref().map_or(true, |x| x != &self.seen[i]) {
        self.current = Some(self.seen[i].clone());
        break;
      }
    }
    Ok(self.current.as_ref().unwrap())
  }

  fn push_strike(&mut self, x: u32) -> Option<&u32>
  {
    for y in self.strike.iter_mut() {
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
      let r = !self.seen.contains(&x) ^ seen;
      self.seen.push(x.clone());
      self.previuos.replace(x);
      if !r {
        self.push_strike(self.count);
      }
      self.count += 1;
      Ok(r)
    } else {
      Err(GameError::EmptyCommit)
    }
  }
}
