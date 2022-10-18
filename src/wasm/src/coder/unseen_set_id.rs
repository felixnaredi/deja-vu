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
}

impl UnseenSetID
{
  pub fn unique_number(&self) -> u64
  {
    use UnseenSetID::*;

    match self {
      Unspecified => 7359453237177161485,
      DictionaryFr01 => 16775286842649692529,
    }
  }
}