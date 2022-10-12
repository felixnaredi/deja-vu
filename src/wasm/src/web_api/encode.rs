use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
  coder::{
    self,
    Version00Coding,
  },
  web_api::History,
};

#[wasm_bindgen]
pub struct Encoded(coder::Encoded);

#[wasm_bindgen]
impl Encoded
{
  #[wasm_bindgen]
  pub fn version(&self) -> String
  {
    self.0.version().into()
  }

  #[wasm_bindgen]
  pub fn data(&self) -> String
  {
    self.0.data().into()
  }

  #[wasm_bindgen]
  pub fn checksum(&self) -> u64
  {
    self.0.checksum()
  }
}

#[wasm_bindgen]
pub fn encode(history: &History) -> Encoded
{
  Encoded(Version00Coding::encode(history.inner()))
}
