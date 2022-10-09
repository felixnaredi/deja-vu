use wasm_bindgen::prelude::{
  wasm_bindgen,
  JsValue,
};

use crate::game;

#[wasm_bindgen]
pub struct Game(game::Game<JsValue>);

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
  pub fn new(seed: u64, seenRatio: f64, unseen: Vec<JsValue>) -> Game
  {
    Game(game::Game::new(seed, seenRatio, unseen.into_iter()))
  }

  /// Generates the next element.
  #[wasm_bindgen]
  pub fn next(&mut self) -> Result<JsValue, String>
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
}
