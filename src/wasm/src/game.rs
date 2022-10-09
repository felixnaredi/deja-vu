use crate::konadare192pxplusplus::Konadare192PxPlusPlus;

const THRESHOLD_MAX: f64 = 1e9;

#[derive(Debug)]
pub struct Game<T>
{
  unseen: Vec<Option<T>>,
  seen: Vec<T>,
  current: Option<T>,
  previuos: Option<T>,
  strike: [Option<u32>; 3],
  seed: u64,
  rng: Konadare192PxPlusPlus,
  seen_threshold: u32,
  count: u32,
}

#[derive(Debug, PartialEq)]
pub enum GameError
{
  UnseenEmpty,
  TooFewSeen,
  EmptyCommit,
  NextCalledWithUncommitedResult,
  GameOver,
  TwiceInARow,
}

impl<T> Game<T>
where
  T: Clone + PartialEq,
{
  /// Create a new game.
  pub fn new(seed: u64, seen_threshold: f64, unseen: impl Iterator<Item = T>) -> Game<T>
  {
    Game {
      unseen: unseen.map(|x| Some(x)).collect(),
      seen: Vec::new(),
      current: None,
      previuos: None,
      strike: [None; 3],
      seed,
      rng: Konadare192PxPlusPlus::from_seed(seed),
      seen_threshold: (seen_threshold * THRESHOLD_MAX).round() as u32,
      count: 0,
    }
  }

  /// True if three wrong commits has been made.
  pub fn game_is_over(&self) -> bool
  {
    self.strike[2].is_some()
  }

  /// Throws `GameError::GameOver` if `game_is_over` is `true`.
  fn game_over(&self) -> Result<(), GameError>
  {
    if self.game_is_over() {
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

// -----------------------------------------------------------------------------
// Test
// -----------------------------------------------------------------------------

#[cfg(test)]
mod test
{
  use std::collections::HashSet;

  use super::*;

  #[test]
  fn commit_errors_when_empty()
  {
    let mut game = Game::new(10539, 0.4, 0..1);
    assert_eq!(game.commit_seen(), Err(GameError::EmptyCommit));
    assert_eq!(game.commit_unseen(), Err(GameError::EmptyCommit));
  }

  #[test]
  fn next_throws_errors_with_uncommited_results()
  {
    let mut game = Game::new(11484, 0.0, 0..2);
    assert!(matches!(game.next(), Ok(_)));
    assert_eq!(game.next(), Err(GameError::NextCalledWithUncommitedResult));

    let mut game = Game::new(11898, 1.0, 0..2);
    assert!(matches!(game.next(), Ok(_)));
    assert_eq!(game.next(), Err(GameError::NextCalledWithUncommitedResult));
  }

  #[test]
  fn three_strikes_guessing_seen_causes_game_over()
  {
    let mut game = Game::new(12584, 0.0, 0..4);
    for _ in 0..3 {
      assert!(matches!(game.next(), Ok(_)));
      assert!(matches!(game.commit_seen(), Ok(false)));
    }
    assert_eq!(game.next(), Err(GameError::GameOver));
    assert_eq!(game.commit_seen(), Err(GameError::GameOver));
    assert_eq!(game.commit_unseen(), Err(GameError::GameOver));
  }

  #[test]
  fn three_strikes_guessing_unseen_causes_game_over()
  {
    let mut game = Game::new(12554, 1.0, 0..4);

    // First two `next` will always be unseen.
    assert!(matches!(game.next(), Ok(_)));
    assert!(matches!(game.commit_unseen(), Ok(true)));
    assert!(matches!(game.next(), Ok(_)));
    assert!(matches!(game.commit_unseen(), Ok(true)));

    for _ in 0..3 {
      assert!(matches!(game.next(), Ok(_)));
      assert!(matches!(game.commit_unseen(), Ok(false)));
    }
    assert_eq!(game.next(), Err(GameError::GameOver));
    assert_eq!(game.commit_seen(), Err(GameError::GameOver));
    assert_eq!(game.commit_unseen(), Err(GameError::GameOver));
  }

  #[test]
  fn next_unseen_returns_unique_and_errors_when_empty()
  {
    let n = 16;
    let mut game = Game::new(6237, 0.0, 0..n);
    let mut s = HashSet::new();

    for _ in 0..n {
      if !s.insert(game.next().unwrap().clone()) {
        panic!("`next_unseen` generated already generated value")
      }
      assert!(game.commit_unseen().unwrap());
    }

    assert_eq!(game.next(), Err(GameError::UnseenEmpty));
  }

  #[test]
  fn next_seen_returns_error_when_too_few_elements()
  {
    let mut game = Game::new(8833, 1.0, 0..0);
    assert!(matches!(game.next(), Err(_)));

    let mut game = Game::new(19119, 1.0, 0..1);
    assert!(matches!(game.next(), Ok(_)));
    assert!(matches!(game.next(), Err(_)));
  }

  #[test]
  fn next_generates_equal_output_for_equal_input()
  {
    let mut game1 = Game::new(10335, 0.5, 0..16);
    let mut game2 = Game::new(10335, 0.5, 0..16);

    for y in [true, true, true, false, false, false, true, true, true] {
      assert_eq!(game1.next(), game2.next());
      if y {
        assert_eq!(game1.commit_seen(), game2.commit_seen());
      } else {
        assert_eq!(game1.commit_unseen(), game2.commit_unseen());
      }
    }
  }

  #[test]
  fn next_never_generates_same_twice_in_a_row()
  {
    let mut game = Game::new(10335, 1.0, 0..4);
    let mut previous = game.next().unwrap().clone();
    for _ in 0..16 {
      game.commit_seen().unwrap();
      let x = game.next().unwrap().clone();
      assert_ne!(previous, x);
      previous = x;
    }
  }
}
