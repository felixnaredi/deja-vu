use wasm_bindgen::prelude::{
  wasm_bindgen,
  JsValue,
};

use crate::{
  game,
  web_api::history::History,
};

#[wasm_bindgen]
pub struct Game(game::Game<String>);

#[wasm_bindgen]
#[allow(non_snake_case)]
impl Game
{
  /// Initialize a new `Game`.
  ///
  /// @param seed Seed used for rng.
  /// @param seenRatio The ratio of seen elements that will be generated.
  /// @param unseen List of unseen values.
  #[wasm_bindgen(constructor)]
  pub fn new(seed: u64, seenRatio: f64, unseen: Vec<JsValue>) -> Result<Game, String>
  {
    Ok(Game(game::Game::new(
      seed,
      seenRatio.try_into().map_err(|e| format!("{}", e))?,
      unseen.into_iter().map(|x| x.as_string().unwrap()).collect(),
    )))
  }

  /// Generates the next element.
  #[wasm_bindgen]
  pub fn next(&mut self) -> Result<String, String>
  {
    self
      .0
      .next()
      .map(|x| x.clone())
      .map_err(|e| format!("{}", e))
  }

  /// Commit the generated element as seen.
  ///
  /// @returns Boolean indicating if the commit was correct.
  #[wasm_bindgen]
  pub fn commitSeen(&mut self) -> Result<bool, String>
  {
    self.0.commit_seen().map_err(|e| format!("{}", e))
  }

  /// Commit the generated element as unseen.
  ///
  /// @returns Boolean indicating if the commit was correct.
  #[wasm_bindgen]
  pub fn commitUnseen(&mut self) -> Result<bool, String>
  {
    self.0.commit_unseen().map_err(|e| format!("{}", e))
  }

  /// The current score.
  ///
  /// @returns The score.
  #[wasm_bindgen]
  pub fn score(&self) -> usize
  {
    self.0.score()
  }

  /// Amount of lives left.
  ///
  /// @returns The amount.
  #[wasm_bindgen]
  pub fn lives(&self) -> usize
  {
    self.0.lives()
  }

  /// The initial amount of lives a `Game` starts with.
  ///
  /// @returns The amount.
  #[wasm_bindgen]
  pub fn initialLivesAmount() -> usize
  {
    game::INITIAL_LIVES_AMOUNT
  }

  #[wasm_bindgen]
  pub fn intoHistory(self) -> History
  {
    History::from(self.0)
  }
}
