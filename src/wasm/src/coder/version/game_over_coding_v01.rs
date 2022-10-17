#[cfg(test)]
mod test;

use std::fmt::Display;

use serde::{
  Deserialize,
  Serialize,
};

use crate::{
  coder::{
    encoded::EncodedGameOver,
    unseen_set_id::UnseenSetID,
    SealedEncodedGameOver,
  },
  game::IncorrectCommits,
  game_over::GameOver,
};

/// Coding that is used to encode a `GameOver` into a `EncodedGameOver` and also decode it.
pub struct GameOverCodingV01;

impl GameOverCodingV01
{
  pub fn encode<T: Serialize + AsRef<[u8]>>(
    game_over: GameOver<T>,
    unseen_set_id: UnseenSetID,
  ) -> Result<SealedEncodedGameOver, GameOverCodingV01Error>
  {
    Err(GameOverCodingV01Error::BadElementChecksum)
  }

  pub fn decode<T>(
    encoded: EncodedGameOver,
    unseen: Vec<T>,
  ) -> Result<GameOver<T>, GameOverCodingV01Error>
  {
    Err(GameOverCodingV01Error::BadElementChecksum)
  }
}

// -------------------------------------------------------------------------------------------------
// Encoded data
// -------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
struct GameOverCodingV01Data
{
  seed: u64,
  unseen_set_id: UnseenSetID,
  incorrect_commits: IncorrectCommits,
  element_checksum: u64,
}

// -------------------------------------------------------------------------------------------------
// Error
// -------------------------------------------------------------------------------------------------

/// Errors thrown when encoding/decoding with `GameOverCodingV01` fails.
#[derive(Debug, PartialEq)]
pub enum GameOverCodingV01Error
{
  BadElementChecksum,
}

impl Display for GameOverCodingV01Error
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    use GameOverCodingV01Error::*;
    match self {
      BadElementChecksum => writeln!(f, "bad element checksum"),
    }
  }
}
