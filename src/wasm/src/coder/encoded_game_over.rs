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
  pub fn new<E, T>(game_over: &GameOver<T>) -> Result<SealedEncodedGameOver, E::Error>
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

// -------------------------------------------------------------------------------------------------
// Encode and decode
// -------------------------------------------------------------------------------------------------

impl TryFrom<SealedEncodedGameOver> for EncodedGameOver
{
  type Error = Box<dyn Error>;

  fn try_from(s: SealedEncodedGameOver) -> Result<EncodedGameOver, Self::Error>
  {
    match GameOverCoderVersion::try_from(&s.version)? {
      GameOverCoderVersion::Version00Coding => Ok(
        ok_checksum::<Version00Coding>(s.checksum, s.data.as_bytes())
          .map(|_| EncodedGameOver(s))?,
      ),

      GameOverCoderVersion::GameOverCoderV01 => Ok(
        ok_checksum::<GameOverCoderV01>(s.checksum, s.data.as_bytes())
          .map(|_| EncodedGameOver(s))?,
      ),
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
    match GameOverCoderVersion::try_from(&s.version)? {
      GameOverCoderVersion::Version00Coding => decode::<Version00Coding, _>(s, unseen),
      GameOverCoderVersion::GameOverCoderV01 => decode::<GameOverCoderV01, _>(s, unseen),
    }
  }
}

//
// Helpers
//

fn ok_checksum<C: CoderChecksum>(checksum: u64, data: &[u8]) -> Result<(), SealedEncodedError>
{
  (checksum == C::checksum(data))
    .then(|| ())
    .ok_or(SealedEncodedError::InvalidChecksum)
}

fn decode<C, T>(s: SealedEncodedGameOver, unseen: Vec<T>) -> Result<GameOver<T>, Box<dyn Error>>
where
  C: CoderChecksum + DecodeGameOver<T, Error = Box<dyn Error>>,
  T: Clone + PartialEq + AsRef<[u8]>,
{
  ok_checksum::<C>(s.checksum, s.data.as_bytes())?;
  C::decode(s.data, s.unseen_set_id, unseen)
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
