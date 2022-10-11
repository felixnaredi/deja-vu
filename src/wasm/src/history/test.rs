#[cfg(test)]
//
//
#[allow(unused_imports)]
use std::collections::LinkedList;

#[allow(unused_imports)]
use crate::{
  game::Game,
  history::{
    History,
    SeenUnseen,
  },
};

#[test]
fn history_is_correct_when_always_commiting_seen()
{
  use SeenUnseen::*;

  let mut game = Game::new(10000179183556691969, 0.5, 0..16);
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

  let history = History::from(game);
  assert_eq!(history.score(), score);
  assert_eq!(history.lives(), lives);

  for (i, commit) in history.into_iter().enumerate() {
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
fn history_is_correct_when_always_commiting_unseen()
{
  use SeenUnseen::*;

  let mut game = Game::new(17230744205331056885, 0.5, 0..16);
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

  let history = History::from(game);
  assert_eq!(history.score(), score);
  assert_eq!(history.lives(), lives);

  for (i, commit) in history.into_iter().enumerate() {
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
fn history_is_correct_for_a_somewhat_realistic_game()
{
  let mut game = Game::new(8217158024524860960, 0.25, 0..64);
  let mut elements = LinkedList::new();
  let guess_seen = [4, 22, 23, 25, 26, 31, 32, 34, 37, 38];
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
  let score = game.score();
  let lives = game.lives();
  let incorrect: Vec<usize> = game
    .incorrect_commits()
    .iter()
    .map(|i| i.as_ref().unwrap())
    .cloned()
    .collect();

  let history = History::from(game);
  assert_eq!(history.score(), score);
  assert_eq!(history.lives(), lives);

  for (i, commit) in history.into_iter().enumerate() {
    assert_eq!(commit.element(), &elements.pop_front().unwrap());
    if incorrect.contains(&i) {
      assert!(!commit.correct())
    } else {
      assert!(commit.correct())
    }
  }

  assert!(elements.is_empty())
}
