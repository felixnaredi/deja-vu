use wasm_bindgen::prelude::wasm_bindgen;

use crate::konadare192pxplusplus::Konadare192PxPlusPlus;

#[wasm_bindgen]
pub struct PRNG(Konadare192PxPlusPlus);

#[wasm_bindgen]
#[allow(non_snake_case)]
impl PRNG
{
  /// Creates a new `PRNG` from `seed`.
  pub fn fromSeed(seed: u64) -> PRNG
  {
    PRNG(Konadare192PxPlusPlus::from_seed(seed))
  }

  /// Next pseudo random number.
  pub fn next(&mut self) -> u64
  {
    self.0.next()
  }

  /// Generates a pseodo random number in the range [0, `upperBound`).
  pub fn nextWithUpperBound(&mut self, upperBound: u32) -> u32
  {
    self.0.next_with_upper_bound(upperBound)
  }
}
