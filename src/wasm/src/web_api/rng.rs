use wasm_bindgen::prelude::wasm_bindgen;

use crate::rng::{
  self,
  IndexedPermutation,
};

// -------------------------------------------------------------------------------------------------
// KSINK
// -------------------------------------------------------------------------------------------------

#[wasm_bindgen]
pub struct KSINK(rng::KSINK);

#[wasm_bindgen]
#[allow(non_snake_case)]
impl KSINK
{
  #[wasm_bindgen]
  pub fn permute(x: u64, c: u64) -> u64
  {
    rng::KSINK::permute_index(x, c)
  }

  #[wasm_bindgen]
  pub fn hash(seed: u64, bytes: &[u8]) -> u64
  {
    rng::KSINK::hash(seed, bytes)
  }

  #[wasm_bindgen]
  pub fn hashString(seed: u64, s: String) -> u64
  {
    rng::KSINK::hash(seed, s.as_bytes())
  }
}

// -------------------------------------------------------------------------------------------------
// Konadare192PxPlusPlus
// -------------------------------------------------------------------------------------------------

#[wasm_bindgen]
pub struct Konadare192PxPlusPlus(rng::Konadare192PxPlusPlus);

#[wasm_bindgen]
#[allow(non_snake_case)]
impl Konadare192PxPlusPlus
{
  /// Creates a new `Konadare192PxPlusPlus` from `seed`.
  #[wasm_bindgen(constructor)]
  pub fn new(seed: u64) -> Konadare192PxPlusPlus
  {
    Konadare192PxPlusPlus(rng::Konadare192PxPlusPlus::from_seed(seed))
  }

  /// Next pseudo random number.
  #[wasm_bindgen]
  pub fn next(&mut self) -> u64
  {
    self.0.next()
  }

  /// Generates a pseodo random number in the range [0, `upper_bound`).
  ///
  /// From Daniel Lemire's "Fast Random Integer Generation in an Interval",
  /// https://arxiv.org/pdf/1805.10941.pdf
  ///
  /// The version below was written by Pelle Evensen.
  #[wasm_bindgen]
  pub fn nextWithUpperBound(&mut self, upper: u32) -> u32
  {
    self.0.next_with_upper_bound(upper)
  }
}
