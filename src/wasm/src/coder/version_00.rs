use std::error::Error;

use serde::{
  Deserialize,
  Serialize,
};

use crate::{
  coder::{
    encoded::{
      Encoded,
      SealedEncoded,
      SealedEncodedBuilder,
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
  pub fn id() -> &'static str
  {
    "00"
  }

  fn base64_encode(self) -> String
  {
    base64::encode(serde_json::to_string(&self).unwrap())
  }

  fn base64_decode(data: &str) -> Result<Self, Box<dyn Error>>
  {
    Ok(serde_json::from_slice(base64::decode(data)?.as_slice())?)
  }

  pub fn hash_seed() -> u64
  {
    4997987866499591411
  }

  pub fn encode<T>(history: &History<T>) -> SealedEncoded
  {
    let version = Self {
      unseen_id: UnseenID::DictionaryFr01,
      seed: history.seed(),
      incorrect_commits: vec![
        history.incorrect_commits()[0].unwrap(),
        history.incorrect_commits()[1].unwrap(),
        history.incorrect_commits()[2].unwrap(),
      ],
    };

    let data = version.base64_encode();

    SealedEncodedBuilder::default()
      .version(Self::id().into())
      .checksum(KNOMUL::hash(Self::hash_seed(), data.as_bytes()))
      .data(data)
      .build()
      .unwrap()
  }

  pub fn decode<T>(encoded: Encoded, unseen: Vec<T>) -> Result<History<T>, Box<dyn Error>>
  where
    T: Clone + PartialEq,
  {
    let decoded = Self::base64_decode(encoded.data())?;
    Ok(History::new(
      decoded.seed,
      unseen,
      0.4.try_into()?,
      [
        Some(decoded.incorrect_commits[0]),
        Some(decoded.incorrect_commits[1]),
        Some(decoded.incorrect_commits[2]),
      ],
    ))
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
    assert_eq!(Version00Coding::base64_decode(&encode).unwrap(), x());
  }
}
