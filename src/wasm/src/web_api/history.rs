use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
  game,
  history,
};

type Inner = String;

/// Pseudo boolean that is either `Seen` or `Unseen`.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SeenUnseen(history::SeenUnseen);

#[wasm_bindgen]
#[allow(non_snake_case)]
impl SeenUnseen
{
  /// True if the state is `Seen`.
  #[wasm_bindgen]
  pub fn isSeen(&self) -> bool
  {
    self.0 == history::SeenUnseen::Seen
  }

  /// True if the state is `Unseen`.
  #[wasm_bindgen]
  pub fn isUnseen(&self) -> bool
  {
    !self.isSeen()
  }
}

/// Type representing a commit made in a game.
#[wasm_bindgen]
#[derive(Clone)]
pub struct Commit(history::Commit<Inner>);

#[wasm_bindgen]
impl Commit
{
  /// The value of the element.
  #[wasm_bindgen]
  pub fn element(&self) -> Inner
  {
    self.0.element().clone()
  }

  /// The actual state of the commit.
  #[wasm_bindgen]
  pub fn actual(&self) -> SeenUnseen
  {
    SeenUnseen(self.0.actual().clone())
  }

  /// The guessed state of the commit.
  #[wasm_bindgen]
  pub fn guess(&self) -> SeenUnseen
  {
    SeenUnseen(self.0.guess().clone())
  }

  /// True if the `guess` is equal to `actual`.
  #[wasm_bindgen]
  pub fn correct(&self) -> bool
  {
    self.0.correct()
  }
}

#[wasm_bindgen]
pub struct History(history::History<Inner>);

impl From<game::Game<Inner>> for History
{
  fn from(game: game::Game<Inner>) -> Self
  {
    History(history::History::from(game))
  }
}

#[allow(non_snake_case)]
#[wasm_bindgen]
impl History
{
  /// Final score of the game.
  #[wasm_bindgen]
  pub fn score(&self) -> usize
  {
    self.0.score()
  }

  /// Final score of the game.
  #[wasm_bindgen]
  pub fn lives(&self) -> usize
  {
    self.0.lives()
  }

  #[wasm_bindgen]
  pub fn intoIterator(self) -> HistoryIterator
  {
    HistoryIterator(self.0.into_iter())
  }
}

#[wasm_bindgen]
pub struct HistoryIterator(history::HistoryIterator<Inner>);

#[wasm_bindgen]
impl HistoryIterator
{
  #[wasm_bindgen]
  pub fn next(&mut self) -> Option<Commit>
  {
    self.0.next().map(|x| Commit(x))
  }
}
