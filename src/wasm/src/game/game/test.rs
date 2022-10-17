use std::collections::{
  HashSet,
  LinkedList,
};

use super::*;
use crate::rng::{
  IndexedPermutation,
  KSINK,
};

#[test]
fn commit_errors_when_empty()
{
  let mut game = Game::new(
    10539,
    0.4.try_into().unwrap(),
    (0..1).map(|x| [x]).collect(),
  );
  assert_eq!(game.commit_seen(), Err(GameError::EmptyCommit));
  assert_eq!(game.commit_unseen(), Err(GameError::EmptyCommit));
}

#[test]
fn next_throws_errors_with_uncommited_results()
{
  let mut game = Game::new(
    11484,
    0.0.try_into().unwrap(),
    (0..2).map(|x| [x]).collect(),
  );
  assert!(matches!(game.next(), Ok(_)));
  assert_eq!(game.next(), Err(GameError::NextCalledWithUncommitedResult));

  let mut game = Game::new(
    11898,
    1.0.try_into().unwrap(),
    (0..2).map(|x| [x]).collect(),
  );
  assert!(matches!(game.next(), Ok(_)));
  assert_eq!(game.next(), Err(GameError::NextCalledWithUncommitedResult));
}

#[test]
fn three_strikes_guessing_seen_causes_game_over()
{
  let mut game = Game::new(
    12584,
    0.0.try_into().unwrap(),
    (0..4).map(|x| [x]).collect(),
  );
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
  let mut game = Game::new(
    12554,
    1.0.try_into().unwrap(),
    (0..4).map(|x| [x]).collect(),
  );

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
  let mut game = Game::new(6237, 0.0.try_into().unwrap(), (0..n).map(|x| [x]).collect());
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
  let mut game = Game::new(8833, 1.0.try_into().unwrap(), (0..0).map(|x| [x]).collect());
  assert!(matches!(game.next(), Err(_)));

  let mut game = Game::new(
    19119,
    1.0.try_into().unwrap(),
    (0..1).map(|x| [x]).collect(),
  );
  assert!(matches!(game.next(), Ok(_)));
  assert!(matches!(game.next(), Err(_)));
}

#[test]
fn next_generates_equal_output_for_equal_input()
{
  let mut game1 = Game::new(
    10335,
    0.5.try_into().unwrap(),
    (0..16).map(|x| [x]).collect(),
  );
  let mut game2 = Game::new(
    10335,
    0.5.try_into().unwrap(),
    (0..16).map(|x| [x]).collect(),
  );

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
  let mut game = Game::new(
    10335,
    1.0.try_into().unwrap(),
    (0..4).map(|x| [x]).collect(),
  );
  let mut previous = game.next().unwrap().clone();
  for _ in 0..16 {
    game.commit_seen().unwrap();
    let x = game.next().unwrap().clone();
    assert_ne!(previous, x);
    previous = x;
  }
}

#[test]
fn score_increases_on_correct_commit()
{
  let mut game = Game::new(
    11976,
    0.5.try_into().unwrap(),
    (0..8).map(|x| [x]).collect(),
  );
  for (score, guess_seen) in [
    (1, false),
    (2, false),
    (2, true),
    (3, false),
    (4, true),
    (4, false),
  ] {
    game.next().unwrap();

    if guess_seen {
      game.commit_seen().unwrap();
    } else {
      game.commit_unseen().unwrap();
    }

    assert_eq!(game.score(), score);
  }
}

#[test]
fn life_decrease_on_incorrect_commit()
{
  let mut game = Game::new(
    211391,
    0.5.try_into().unwrap(),
    (0..8).map(|x| [x]).collect(),
  );
  for (lives, guess_seen) in [
    (3, false),
    (3, false),
    (3, true),
    (3, false),
    (2, true),
    (2, true),
    (2, false),
    (1, true),
    (1, false),
    (0, false),
  ] {
    game.next().unwrap();

    if guess_seen {
      game.commit_seen().unwrap();
    } else {
      game.commit_unseen().unwrap();
    }
    assert_eq!(game.lives(), lives);
  }

  assert!(game.finished());
}

#[test]
fn indices_are_same_as_incorrect_commits()
{
  let mut game = Game::new(
    877326994,
    0.5.try_into().unwrap(),
    (0..16).map(|x| [x]).collect(),
  );
  let guess_seen = [2, 3, 5, 6, 11];
  let wrongs = [3, 8, 11];

  for i in 0..12 {
    game.next().unwrap();

    if guess_seen.contains(&i) {
      assert!(game.commit_seen().unwrap() == !wrongs.contains(&i));
    } else {
      assert!(game.commit_unseen().unwrap() == !wrongs.contains(&i));
    }
  }

  assert_eq!(
    wrongs
      .iter()
      .map(|x| Some(x.clone()))
      .collect::<Vec<Option<usize>>>(),
    game.incorrect_commits()
  );
}

#[test]
fn after_reseting_a_game_it_produces_same_output_given_same_input()
{
  let mut game = Game::new(
    6314949274223572360,
    0.4.try_into().unwrap(),
    (0..32).map(|x| [x]).collect(),
  );
  let guess_seen = [4, 5, 8, 12, 17, 18, 19, 20, 21, 22, 24, 25];

  let mut elements = LinkedList::new();
  let mut i = 0;
  while !game.finished() {
    elements.push_back(game.next().unwrap().clone());
    if guess_seen.contains(&i) {
      game.commit_seen().unwrap();
    } else {
      game.commit_unseen().unwrap();
    }
    i += 1;
  }

  let incorrect = game.incorrect_commits().clone();

  game.reset();
  let mut i = 0;
  while !game.finished() {
    assert_eq!(game.next().unwrap(), &elements.pop_front().unwrap());
    if guess_seen.contains(&i) {
      game.commit_seen().unwrap();
    } else {
      game.commit_unseen().unwrap();
    }
    i += 1;
  }

  assert!(elements.is_empty());
  assert_eq!(game.incorrect_commits(), incorrect);
}

#[test]
fn element_checksum_outputs_as_expected()
{
  let mut game = Game::new(
    9004491465204253423,
    0.3.try_into().unwrap(),
    vec![
      "4hYKvDtHGSo=",
      "/QaJrz+CL/Q=",
      "d6z9AveYhgY=",
      "3t69Oqkp/DM=",
      "1/sKN1Lxhb8=",
      "MXe8R8hcGds=",
      "ZOIvjnNgSgo=",
      "XF1nMT3+sNI=",
      "l5a/lq691wc=",
      "5ogqZBBRKAc=",
      "PMVOmOlhnRc=",
      "7jAprh4MM34=",
      "M1q/CZxTsMU=",
      "vGMrB1HuhnM=",
      "SPYz7ePBqJU=",
      "8O/quTDz7zw=",
    ],
  );

  let mut s = DEFAULT_ELEMENT_CHECKSUM;
  assert_eq!(s, game.element_checksum());

  let mut m = 0;

  while !game.finished() {
    s = KSINK::hash(s, game.next().unwrap().as_bytes());
    assert_eq!(s, game.element_checksum());

    m += 1;
    m %= 3;
    if m == 0 {
      game.commit_seen().unwrap();
    } else {
      game.commit_unseen().unwrap();
    }
  }
}
