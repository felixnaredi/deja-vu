use std::error::Error;

use serde::{
  Deserialize,
  Serialize,
};

use crate::{
  coder::{
    encoded::{
      Encoded,
      EncodedBuilder,
    },
    unseen_id::UnseenID,
  },
  history::History,
  rng::KNOMUL,
};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Version00Coding
{
  unseen_id: UnseenID,
  seed: u64,
  incorrect_commits: Vec<usize>,
}

impl Version00Coding
{
  fn id() -> &'static str
  {
    "00"
  }

  fn base64_encode(self) -> String
  {
    base64::encode(serde_json::to_string(&self).unwrap())
  }

  fn base64_decode(data: String) -> Result<Self, Box<dyn Error>>
  {
    Ok(serde_json::from_slice(base64::decode(data)?.as_slice())?)
  }

  fn hash_seed() -> u64
  {
    4997987866499591411
  }

  pub fn encode<T>(history: &History<T>) -> Encoded
  where
    T: Serialize,
  {
    let version = Self {
      unseen_id: UnseenID::DictionaryFr01,
      seed: history.seed(),
      incorrect_commits: history.incorrect_commits(),
    };

    let data = version.base64_encode();

    EncodedBuilder::default()
      .version(Self::id().into())
      .checksum(KNOMUL::hash(Self::hash_seed(), data.as_bytes()))
      .data(data)
      .build()
      .unwrap()
  }
}

#[cfg(test)]
mod test
{
  use super::*;

  #[test]
  fn encode_decode_equals_original()
  {
    let x = || Version00Coding {
      unseen_id: UnseenID::DictionaryFr01,
      seed: 3732249406730752636,
      incorrect_commits: vec![1, 5, 8],
    };
    let encode = x().base64_encode();
    assert_eq!(Version00Coding::base64_decode(encode).unwrap(), x());
  }
}
