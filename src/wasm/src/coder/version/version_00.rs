use std::error::Error;

use serde::{
  Deserialize,
  Serialize,
};

use crate::{
  coder::{
    encoded::{
      EncodedGameOver,
      SealedEncodedGameOver,
      SealedEncodedGameOverBuilder,
    },
    unseen_id::UnseenSetID,
  },
  game_over::GameOver,
  rng::{
    IndexedPermutation,
    KNOMUL,
  },
};

/// `Version00Coding` is a simple coding format that can restore played games.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Version00Coding
{
  unseen_id: UnseenSetID,
  seed: u64,
  incorrect_commits: Vec<usize>,
}

impl Version00Coding
{
  /// Version id.
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

  /// Seed used when hashing `data` for the checksum of the encoding.
  pub fn hash_seed() -> u64
  {
    4997987866499591411
  }

  /// Encodes `game_over`.
  pub fn encode<T>(game_over: &GameOver<T>) -> SealedEncodedGameOver
  {
    let version = Self {
      unseen_id: UnseenSetID::DictionaryFr01,
      seed: game_over.seed(),
      incorrect_commits: vec![
        game_over.incorrect_commits()[0].unwrap(),
        game_over.incorrect_commits()[1].unwrap(),
        game_over.incorrect_commits()[2].unwrap(),
      ],
    };

    let data = version.base64_encode();

    SealedEncodedGameOverBuilder::default()
      .version(Self::id().into())
      .checksum(KNOMUL::hash(Self::hash_seed(), data.as_bytes()))
      .data(data)
      .build()
      .unwrap()
  }

  /// Decodes `encoded`, restoring the encoded `GameOver<T>`. Fails if the serialized `data` is
  /// currupt.
  pub fn decode<T>(encoded: EncodedGameOver, unseen: Vec<T>) -> Result<GameOver<T>, Box<dyn Error>>
  where
    T: Clone + PartialEq + AsRef<[u8]>,
  {
    let decoded = Self::base64_decode(encoded.data())?;
    Ok(GameOver::new(
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

// -------------------------------------------------------------------------------------------------
// Test
// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test
{
  use std::iter;

  use super::*;

  #[test]
  fn data_can_be_base64_encoded_and_decoded()
  {
    let x = || Version00Coding {
      unseen_id: UnseenSetID::DictionaryFr01,
      seed: 3732249406730752636,
      incorrect_commits: vec![36, 40, 57],
    };
    let encode = x().base64_encode();
    assert_eq!(Version00Coding::base64_decode(&encode).unwrap(), x());
  }

  #[test]
  fn encode_decode_is_equal_to_id_for_game_over()
  {
    let unseen: Vec<[u8; 1]> = (0..64).map(|x| [x]).collect();

    let game_over = GameOver::new(
      7789954068733337566,
      unseen.clone(),
      0.4.try_into().unwrap(),
      [Some(36), Some(40), Some(57)],
    );
    let encoded = Version00Coding::encode(&game_over);
    let decoded = Version00Coding::decode(encoded.try_into().unwrap(), unseen).unwrap();

    let mut i = 0;
    for (c0, c1) in iter::zip(game_over.into_iter(), decoded.into_iter()) {
      assert_eq!(c0, c1);
      i += 1;
    }
    assert_eq!(i, 58);
  }
}
