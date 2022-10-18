use std::fmt::Display;

use serde::{
  Deserialize,
  Serialize,
};

use crate::{
  coder::{
    encoded::GameOverCoder,
    unseen_set_id::UnseenSetID,
  },
  game::{
    IncorrectCommits,
    SeenThreshold,
  },
  game_over::GameOver,
  rng::{
    IndexedPermutation,
    KSINK,
  },
};

const VERSION: &'static str = "goc-v01";
const SEED: u64 = 9375103332589136009;

/// Coder that is used to encode a `GameOver` into a `EncodedGameOver` and also decode it.
pub struct GameOverCoderV01;

impl GameOverCoder for GameOverCoderV01
{
  type Error = Box<dyn std::error::Error>;

  fn version() -> &'static str
  {
    VERSION
  }

  fn checksum(data: &[u8]) -> u64
  {
    KSINK::hash(SEED, data)
  }

  fn encode<T>(game_over: &GameOver<T>) -> Result<String, Self::Error>
  {
    Ok(base64::encode(&serde_json::to_string(
      &GameOverCoderV01Data {
        seed: game_over.seed(),
        unseen_set_id: game_over.unseen_set_id().clone(),
        seen_threshold: game_over.seen_threshold(),
        incorrect_commits: game_over.incorrect_commits(),
        element_checksum: game_over.element_checksum(),
      },
    )?))
  }

  fn decode<T: PartialEq + Clone + AsRef<[u8]>>(
    data: String,
    unseen: Vec<T>,
  ) -> Result<GameOver<T>, Self::Error>
  {
    let data = base64::decode(data)?;
    let data: GameOverCoderV01Data = serde_json::from_slice(&data)?;
    let game_over = GameOver::new(
      data.seed,
      data.unseen_set_id,
      unseen,
      data.seen_threshold,
      data.incorrect_commits,
    );
    if data.element_checksum != game_over.element_checksum() {
      Err(Box::new(GameOverCoderV01Error::BadElementChecksum))
    } else {
      Ok(game_over)
    }
  }
}

// -------------------------------------------------------------------------------------------------
// Encoded data
// -------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
struct GameOverCoderV01Data
{
  seed: u64,
  unseen_set_id: UnseenSetID,
  seen_threshold: SeenThreshold,
  incorrect_commits: IncorrectCommits,
  element_checksum: u64,
}

// -------------------------------------------------------------------------------------------------
// Error
// -------------------------------------------------------------------------------------------------

/// Errors thrown when encoding/decoding with `GameOverCodingV01` fails.
#[derive(Debug)]
pub enum GameOverCoderV01Error
{
  BadElementChecksum,
}

impl Display for GameOverCoderV01Error
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    use GameOverCoderV01Error::*;
    match self {
      BadElementChecksum => writeln!(f, "bad element checksum"),
    }
  }
}

impl std::error::Error for GameOverCoderV01Error {}
