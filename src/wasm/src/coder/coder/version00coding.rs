use std::error::Error;

use serde::{
  Deserialize,
  Serialize,
};

use crate::{
  coder::{
    encoded_game_over::{
      CoderChecksum,
      CoderVersion,
      DecodeGameOver,
      EncodeGameOver,
    },
    unseen_set_id::UnseenSetID,
    version::GameOverCoderVersion,
  },
  game_over::GameOver,
  rng::{
    IndexedPermutation,
    KNOMUL,
  },
};

const SEED: u64 = 4997987866499591411;

/// `Version00Coding` is a simple coding format that can restore played games.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Version00Coding
{
  unseen_id: UnseenSetID,
  seed: u64,
  incorrect_commits: Vec<usize>,
}

// -------------------------------------------------------------------------------------------------
// Coder Implementation
// -------------------------------------------------------------------------------------------------

impl CoderVersion for Version00Coding
{
  fn version() -> GameOverCoderVersion
  {
    GameOverCoderVersion::Version00Coding
  }
}

impl CoderChecksum for Version00Coding
{
  fn checksum(data: &[u8]) -> u64
  {
    KNOMUL::hash(SEED, data)
  }
}

impl<T> EncodeGameOver<T> for Version00Coding
{
  type Error = Box<dyn Error>;

  fn encode(game_over: &GameOver<T>) -> Result<String, Self::Error>
  {
    Ok(base64::encode(serde_json::to_string(&Self {
      unseen_id: UnseenSetID::DictionaryFr01,
      seed: game_over.seed(),
      incorrect_commits: vec![
        game_over.incorrect_commits()[0].unwrap(),
        game_over.incorrect_commits()[1].unwrap(),
        game_over.incorrect_commits()[2].unwrap(),
      ],
    })?))
  }
}

impl<T> DecodeGameOver<T> for Version00Coding
where
  T: Clone + AsRef<[u8]> + PartialEq,
{
  type Error = Box<dyn Error>;

  fn decode(
    data: String,
    _unseen_set_id: UnseenSetID,
    unseen: Vec<T>,
  ) -> Result<GameOver<T>, Self::Error>
  {
    let decoded: Version00Coding = serde_json::from_slice(base64::decode(data)?.as_slice())?;
    Ok(GameOver::new(
      decoded.seed,
      UnseenSetID::DictionaryFr01,
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
