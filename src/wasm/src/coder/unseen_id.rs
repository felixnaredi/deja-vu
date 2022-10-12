use serde::{
  Deserialize,
  Serialize,
};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum UnseenID
{
  DictionaryFr01,
}
