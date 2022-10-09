#[cfg(test)]
use std::collections::HashSet;

#[allow(unused_imports)]
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
