use wasm_bindgen::prelude::wasm_bindgen;

use crate::coder;

/// The ID of a unseen set.
#[derive(Debug, PartialEq)]
#[wasm_bindgen]
pub enum UnseenSetIDPrimitive
{
  /// Can be used for tests.
  Unspecified,

  /// French...
  DictionaryFr01,

  /// The top 999 most used french words according to [Wiktionary](https://en.wiktionary.org/wiki/Wiktionary:Frequency_lists/French_wordlist_opensubtitles_5000).
  Top999WiktionaryFr,
}

// -------------------------------------------------------------------------------------------------
// Enable cast to `coder::UnseenSetID`
// -------------------------------------------------------------------------------------------------

impl From<UnseenSetIDPrimitive> for coder::UnseenSetID
{
  fn from(id: UnseenSetIDPrimitive) -> Self
  {
    use UnseenSetIDPrimitive::*;

    match id {
      Unspecified => coder::UnseenSetID::Unspecified,
      DictionaryFr01 => coder::UnseenSetID::DictionaryFr01,
      Top999WiktionaryFr => coder::UnseenSetID::Top999WiktionaryFr,
    }
  }
}
