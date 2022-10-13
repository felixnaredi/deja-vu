use serde::{
  Deserialize,
  Serialize,
};

/// ID of the set that elements are picked from.
///
/// TODO:
///   This is currently ad-hoc set to `UnseenID::DictionaryFr01`, both in frontend and backend.
///   Focus on generalization is not relevant until more than one playable set exists.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum UnseenID
{
  DictionaryFr01,
}
