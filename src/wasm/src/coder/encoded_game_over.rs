#[cfg(test)]
mod test;

use std::{
  error::Error,
  fmt::Display,
};

use serde::{
  Deserialize,
  Serialize,
};

use super::{
  version::GameOverCoderVersion,
  UnseenSetID,
};
use crate::{
  coder::{
    GameOverCoderV01,
    Version00Coding,
  },
  game_over::GameOver,
  rng::{
    IndexedPermutation,
    KNOMUL,
  },
};

// -------------------------------------------------------------------------------------------------
// Coder
// -------------------------------------------------------------------------------------------------

pub trait CoderVersion
{
  fn version() -> GameOverCoderVersion;
}

pub trait CoderChecksum
{
  fn checksum(data: &[u8]) -> u64;
}

pub trait EncodeGameOver<T>
{
  type Error;
  fn encode(game_over: &GameOver<T>) -> Result<String, Self::Error>;
}

pub trait DecodeGameOver<T>
{
  type Error;
  fn decode(
    data: String,
    unseen_set_id: UnseenSetID,
    unseen: Vec<T>,
  ) -> Result<GameOver<T>, Self::Error>;
}

// -------------------------------------------------------------------------------------------------
// Encoded
// -------------------------------------------------------------------------------------------------

/// Access to the encoded `data`.
#[derive(Debug)]
pub struct EncodedGameOver(SealedEncodedGameOver);

impl EncodedGameOver
{
  /// Base64 encoded data.
  pub fn data(&self) -> &str
  {
    self.0.data.as_ref()
  }

  /// The `UnseenSetID` of the game.
  pub fn unseen_set_id(&self) -> &UnseenSetID
  {
    &self.0.unseen_set_id
  }
}

// -------------------------------------------------------------------------------------------------
// SealedEncoded
// -------------------------------------------------------------------------------------------------

fn default_unseen_set_id() -> UnseenSetID
{
  UnseenSetID::DictionaryFr01
}

/// The container of data and meta data for an `Encoded`. To access the underlying data it can be
/// cast into an `Encoded` with `SealedEncoded::try_into()`.
#[derive(Builder, Clone, Debug, Serialize, Deserialize)]
pub struct SealedEncodedGameOver
{
  version: String,
  checksum: u64,
  data: String,

  #[serde(default = "default_unseen_set_id")]
  unseen_set_id: UnseenSetID,
}

impl SealedEncodedGameOver
{
  // TODO:
  //   This function is missing tests.
  pub fn new<E, T>(game_over: GameOver<T>) -> Result<SealedEncodedGameOver, E::Error>
  where
    E: CoderVersion + CoderChecksum + EncodeGameOver<T>,
  {
    E::encode(&game_over).map(|data| SealedEncodedGameOver {
      version: E::version().into(),
      checksum: E::checksum(data.as_bytes()),
      data,
      unseen_set_id: game_over.unseen_set_id().clone(),
    })
  }
}

impl TryFrom<SealedEncodedGameOver> for EncodedGameOver
{
  type Error = Box<dyn Error>;

  fn try_from(s: SealedEncodedGameOver) -> Result<EncodedGameOver, Self::Error>
  {
    use SealedEncodedError::*;

    match GameOverCoderVersion::try_from(&s.version)? {
      GameOverCoderVersion::Version00Coding => {
        if s.checksum == KNOMUL::hash(Version00Coding::hash_seed(), s.data.as_bytes()) {
          Ok(EncodedGameOver(s))
        } else {
          Err(InvalidChecksum)?
        }
      }
      GameOverCoderVersion::GameOverCoderV01 => {
        if s.checksum == GameOverCoderV01::checksum(s.data.as_bytes()) {
          Ok(EncodedGameOver(s))
        } else {
          Err(InvalidChecksum)?
        }
      }
    }
  }
}

impl<T> TryFrom<(SealedEncodedGameOver, Vec<T>)> for GameOver<T>
where
  T: Clone + PartialEq + AsRef<[u8]>,
{
  type Error = Box<dyn std::error::Error>;

  fn try_from((s, unseen): (SealedEncodedGameOver, Vec<T>)) -> Result<GameOver<T>, Self::Error>
  {
    use SealedEncodedError::*;

    match GameOverCoderVersion::try_from(&s.version)? {
      GameOverCoderVersion::Version00Coding => {
        if s.checksum == KNOMUL::hash(Version00Coding::hash_seed(), s.data.as_bytes()) {
          Ok(Version00Coding::decode(EncodedGameOver(s), unseen)?)
        } else {
          Err(Box::new(InvalidChecksum))
        }
      }
      GameOverCoderVersion::GameOverCoderV01 => {
        if s.checksum == GameOverCoderV01::checksum(s.data.as_bytes()) {
          Ok(GameOverCoderV01::decode(s.data, s.unseen_set_id, unseen)?)
        } else {
          Err(Box::new(InvalidChecksum))
        }
      }
    }
  }
}

// -------------------------------------------------------------------------------------------------
// SealedEncodedError
// -------------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub enum SealedEncodedError
{
  InvalidChecksum,
}

impl Display for SealedEncodedError
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    use SealedEncodedError::*;

    match self {
      InvalidChecksum => writeln!(f, "the data is currupted"),
    }
  }
}

impl std::error::Error for SealedEncodedError {}
