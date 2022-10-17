use serde::{
  Deserialize,
  Serialize,
};

/// ID of the set that elements are picked from.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum UnseenSetID
{
  /// Can be used for tests.
  Unspecified,

  /// French...
  DictionaryFr01,
}
