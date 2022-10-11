use serde::Serialize;

use crate::game::{
  Game,
  INITIAL_LIVES_AMOUNT,
};

// -------------------------------------------------------------------------------------------------
// Commit
// -------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum SeenUnseen
{
  Seen,
  Unseen,
}

/// A commit.
///
/// TODO:
///   It should be possible for `element` to have type `&T`. I don't know if it will matter for the
///   web api.
#[derive(Clone, Debug, Serialize)]
pub struct Commit<T>
{
  element: T,
  actual: SeenUnseen,
  guess: SeenUnseen,
}

impl<T> Commit<T>
{
  /// The element generated for the commit.
  pub fn element(&self) -> &T
  {
    &self.element
  }

  /// Actual state of the commit.
  pub fn actual(&self) -> &SeenUnseen
  {
    &self.actual
  }

  /// Guessed state of the commit.
  pub fn guess(&self) -> &SeenUnseen
  {
    &self.guess
  }

  /// Is true if the actual state is equal to the guess.
  pub fn correct(&self) -> bool
  {
    self.actual == self.guess
  }
}

// -------------------------------------------------------------------------------------------------
// History
// -------------------------------------------------------------------------------------------------

/// Represents a already played `Game`.
#[derive(Debug)]
pub struct History<T>(Game<T>);

impl<T> From<Game<T>> for History<T>
{
  fn from(game: Game<T>) -> Self
  {
    History(game)
  }
}

impl<T> History<T>
{
  /// Final score of the game.
  pub fn score(&self) -> usize
  {
    self.0.score()
  }

  /// Lives left when the game finished.
  ///
  /// NOTE:
  ///   This is more for 'correctness' than actual value. The returned value will be 0 in most
  ///   practical cases.
  pub fn lives(&self) -> usize
  {
    self.0.lives()
  }
}

// -------------------------------------------------------------------------------------------------
// Iterator
// -------------------------------------------------------------------------------------------------

/// Iterates over the commits done in a `Game`.
#[derive(Debug)]
pub struct HistoryIterator<T>
{
  game: Game<T>,
  index: usize,
  incorrect_commits: [Option<usize>; INITIAL_LIVES_AMOUNT],
  seen: Vec<T>,
}

impl<T> Iterator for HistoryIterator<T>
where
  T: Clone + PartialEq,
{
  type Item = Commit<T>;

  fn next(&mut self) -> Option<Self::Item>
  {
    use SeenUnseen::*;

    if self.game.finished() {
      None
    } else {
      let element = self.game.next().unwrap().clone();

      if self.seen.contains(&element) {
        Some(Commit {
          element,
          actual: Seen,
          guess: if self.incorrect_commits.contains(&Some(self.index)) {
            self.index += 1;
            self.game.commit_unseen().unwrap();
            Unseen
          } else {
            self.index += 1;
            self.game.commit_seen().unwrap();
            Seen
          },
        })
      } else {
        self.seen.push(element.clone());
        Some(Commit {
          element,
          actual: Unseen,
          guess: if self.incorrect_commits.contains(&Some(self.index)) {
            self.index += 1;
            self.game.commit_seen().unwrap();
            Seen
          } else {
            self.index += 1;
            self.game.commit_unseen().unwrap();
            Unseen
          },
        })
      }
    }
  }
}

impl<T> History<T>
where
  T: Clone + PartialEq,
{
  pub fn into_iter(self) -> HistoryIterator<T>
  {
    let incorrect_commits = self.0.incorrect_commits().clone();
    HistoryIterator {
      game: self.0.reset(),
      index: 0,
      incorrect_commits,
      seen: Vec::new(),
    }
  }
}
