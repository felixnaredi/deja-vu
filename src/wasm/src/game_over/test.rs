use std::{
  collections::LinkedList,
  iter,
};

use crate::{
  game::{
    Game,
    INITIAL_LIVES_AMOUNT,
  },
  game_over::{
    GameOver,
    SeenUnseen,
  },
};

fn run_game<T>(game: &mut Game<T>, guess_seen: &[usize]) -> Vec<T>
where
  T: Clone + PartialEq,
{
  let mut s = Vec::new();
  let mut i = 0;
  while !game.finished() {
    s.push(game.next().unwrap().clone());
    if guess_seen.contains(&i) {
      game.commit_seen().unwrap();
    } else {
      game.commit_unseen().unwrap();
    }
    i += 1;
  }
  s
}

#[test]
fn correct_when_always_commiting_seen()
{
  use SeenUnseen::*;

  let mut game = Game::new(
    10000179183556691969,
    0.5.try_into().unwrap(),
    (0..16).collect(),
  );
  let mut elements = LinkedList::new();

  while !game.finished() {
    elements.push_back(game.next().unwrap().clone());
    game.commit_seen().unwrap();
  }
  let score = game.score();
  let lives = game.lives();
  let incorrect: Vec<usize> = game
    .incorrect_commits()
    .iter()
    .map(|i| i.as_ref().unwrap())
    .cloned()
    .collect();

  let game_over = GameOver::from(game);
  assert_eq!(game_over.score(), score);
  assert_eq!(game_over.lives(), lives);

  for (i, commit) in game_over.into_iter().enumerate() {
    assert_eq!(commit.guess(), &Seen);
    assert_eq!(commit.element(), &elements.pop_front().unwrap());

    if incorrect.contains(&i) {
      assert_eq!(commit.actual(), &Unseen);
      assert!(!commit.correct())
    } else {
      assert_eq!(commit.actual(), &Seen);
      assert!(commit.correct())
    }
  }

  assert!(elements.is_empty())
}

#[test]
fn correct_when_always_commiting_unseen()
{
  use SeenUnseen::*;

  let mut game = Game::new(
    17230744205331056885,
    0.5.try_into().unwrap(),
    (0..16).collect(),
  );
  let mut elements = LinkedList::new();

  while !game.finished() {
    elements.push_back(game.next().unwrap().clone());
    game.commit_unseen().unwrap();
  }
  let score = game.score();
  let lives = game.lives();
  let incorrect: Vec<usize> = game
    .incorrect_commits()
    .iter()
    .map(|i| i.as_ref().unwrap())
    .cloned()
    .collect();

  let game_over = GameOver::from(game);
  assert_eq!(game_over.score(), score);
  assert_eq!(game_over.lives(), lives);

  for (i, commit) in game_over.into_iter().enumerate() {
    assert_eq!(commit.guess(), &Unseen);
    assert_eq!(commit.element(), &elements.pop_front().unwrap());

    if incorrect.contains(&i) {
      assert_eq!(commit.actual(), &Seen);
      assert!(!commit.correct())
    } else {
      assert_eq!(commit.actual(), &Unseen);
      assert!(commit.correct())
    }
  }

  assert!(elements.is_empty())
}

#[test]
fn correct_for_a_somewhat_realistic_game()
{
  let mut game = Game::new(
    8217158024524860960,
    0.25.try_into().unwrap(),
    (0..64).collect(),
  );
  let elements = run_game(&mut game, &[4, 22, 23, 25, 26, 31, 32, 34, 37, 38]);

  let score = game.score();
  let lives = game.lives();
  let incorrect: Vec<usize> = game
    .incorrect_commits()
    .iter()
    .map(|i| i.as_ref().unwrap())
    .cloned()
    .collect();

  let game_over = GameOver::from(game);
  assert_eq!(game_over.score(), score);
  assert_eq!(game_over.lives(), lives);

  let mut lives = INITIAL_LIVES_AMOUNT;

  for (i, (commit, element)) in iter::zip(game_over.into_iter(), elements.iter()).enumerate() {
    assert_eq!(commit.element(), element);
    if incorrect.contains(&i) {
      assert!(!commit.correct());
      lives -= 1;
    } else {
      assert!(commit.correct());
    }
  }
  assert_eq!(lives, 0);
}

#[test]
fn can_be_iterated_over_multiple_times()
{
  let mut game = Game::new(
    10648384310693818260,
    0.4.try_into().unwrap(),
    (0..32).collect(),
  );
  let elements = run_game(&mut game, &[3, 7, 8, 13, 21, 23, 24]);

  let game_over = GameOver::from(game);

  for (commit, element) in iter::zip(game_over.iter(), elements.iter()) {
    assert_eq!(commit.element(), element)
  }
  for (commit, element) in iter::zip(game_over.iter(), elements.iter()) {
    assert_eq!(commit.element(), element)
  }
}

#[test]
fn initialized_with_new_works()
{
  let game_over = GameOver::new(
    13287618919374043026,
    (0..64).collect(),
    0.5.try_into().unwrap(),
    [Some(14), Some(22), Some(35)],
  );
  assert_eq!(game_over.score(), 33);
  assert_eq!(game_over.lives(), 0);
  assert_eq!(
    game_over.incorrect_commits(),
    [Some(14), Some(22), Some(35)]
  );
}
