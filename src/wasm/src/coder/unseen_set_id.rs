use serde::{
  Deserialize,
  Serialize,
};

/// ID of the set that elements are picked from.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum UnseenSetID
{
  /// Can be used for tests.
  Unspecified,

  /// French...
  DictionaryFr01,

  /// The top 999 most used french words according to [Wiktionary](https://en.wiktionary.org/wiki/Wiktionary:Frequency_lists/French_wordlist_opensubtitles_5000).
  Top999WiktionaryFr,
}

impl UnseenSetID
{
  /// Unique number for each value of `&self`.
  pub fn unique_number(&self) -> u64
  {
    use UnseenSetID::*;

    match self {
      Unspecified => 7359453237177161485,
      DictionaryFr01 => 16775286842649692529,
      Top999WiktionaryFr => 4682054772874934823,
    }
  }
}
