use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
  game,
  game_over,
};

type Inner = String;

/// Pseudo boolean that is either `Seen` or `Unseen`.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SeenUnseen(game_over::SeenUnseen);

#[wasm_bindgen]
#[allow(non_snake_case)]
impl SeenUnseen
{
  /// True if the state is `Seen`.
  #[wasm_bindgen]
  pub fn isSeen(&self) -> bool
  {
    self.0 == game_over::SeenUnseen::Seen
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
pub struct Commit(game_over::Commit<Inner>);

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
pub struct GameOver(game_over::GameOver<Inner>);

impl GameOver
{
  pub fn inner(&self) -> &game_over::GameOver<Inner>
  {
    &self.0
  }
}

impl From<game::Game<Inner>> for GameOver
{
  fn from(game: game::Game<Inner>) -> Self
  {
    GameOver(game_over::GameOver::from(game))
  }
}

impl From<game_over::GameOver<Inner>> for GameOver
{
  fn from(game_over: game_over::GameOver<Inner>) -> Self
  {
    GameOver(game_over)
  }
}

#[allow(non_snake_case)]
#[wasm_bindgen]
impl GameOver
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
  pub fn iterator(&self) -> GameOverIterator
  {
    GameOverIterator(self.0.iter())
  }
}

#[wasm_bindgen]
pub struct GameOverIterator(game_over::GameOverIterator<Inner>);

#[wasm_bindgen]
impl GameOverIterator
{
  #[wasm_bindgen]
  pub fn next(&mut self) -> Option<Commit>
  {
    self.0.next().map(|x| Commit(x))
  }
}
