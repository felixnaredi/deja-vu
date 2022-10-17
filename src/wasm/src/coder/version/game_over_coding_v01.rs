use serde::{
  Deserialize,
  Serialize,
};

use crate::{
  coder::unseen_id::UnseenID,
  game::IncorrectCommits,
  EncodedGameOver,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameOverCodingV01Data
{
  seed: u64,
  unseen_set_id: UnseenID,
  incorrect_commits: IncorrectCommits,
  elements_checksum: u64,
}

/// Coding that is used to encode a `GameOver` into a `EncodedGameOver` and also decode it.
pub struct GameOverCodingV01;

impl GameOverCodingV01
{
  // pub fn encode<T: Serialize>(unseen_set_id: UnseenID, unseen: Vec<T>) -> EncodedGameOver {}
}
