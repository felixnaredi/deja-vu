#[cfg(test)]
mod test;

use std::fmt::Display;

use serde::{
  Deserialize,
  Serialize,
};

use super::UnseenSetID;
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

pub trait GameOverCoder
{
  type Error;

  fn version() -> &'static str;
  fn checksum(data: &[u8]) -> u64;
  fn encode<T>(game_over: &GameOver<T>) -> Result<String, Self::Error>;

  // TODO:
  //   Instead of having lots of requirements on `T` here, it might be better to let `T` be a trait
  //   bound type and let the implementing type narrow down what `T` it can decode.
  fn decode<T: PartialEq + Clone + AsRef<[u8]>>(
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
  pub fn new<E: GameOverCoder, T>(
    game_over: &GameOver<T>,
  ) -> Result<SealedEncodedGameOver, E::Error>
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
  type Error = SealedEncodedError;

  fn try_from(s: SealedEncodedGameOver) -> Result<EncodedGameOver, SealedEncodedError>
  {
    use SealedEncodedError::*;

    match s.version.as_str() {
      v if v == Version00Coding::id() => {
        if s.checksum == KNOMUL::hash(Version00Coding::hash_seed(), s.data.as_bytes()) {
          Ok(EncodedGameOver(s))
        } else {
          Err(InvalidChecksum)
        }
      }
      v if v == GameOverCoderV01::version() => {
        if s.checksum == GameOverCoderV01::checksum(s.data.as_bytes()) {
          Ok(EncodedGameOver(s))
        } else {
          Err(InvalidChecksum)
        }
      }
      _ => Err(UnrecognisedVersion(s.version)),
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

    if s.version == Version00Coding::id() {
      if s.checksum == KNOMUL::hash(Version00Coding::hash_seed(), s.data.as_bytes()) {
        Ok(Version00Coding::decode(EncodedGameOver(s), unseen)?)
      } else {
        Err(Box::new(InvalidChecksum))
      }
    } else if s.version == GameOverCoderV01::version() {
      if s.checksum == GameOverCoderV01::checksum(s.data.as_bytes()) {
        Ok(GameOverCoderV01::decode(s.data, s.unseen_set_id, unseen)?)
      } else {
        Err(Box::new(InvalidChecksum))
      }
    } else {
      Err(Box::new(UnrecognisedVersion(s.version)))
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
  UnrecognisedVersion(String),
}

impl Display for SealedEncodedError
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    use SealedEncodedError::*;

    match self {
      InvalidChecksum => writeln!(f, "the data is currupted"),
      UnrecognisedVersion(s) => writeln!(f, "version '{}' is not recognised", s),
    }
  }
}

impl std::error::Error for SealedEncodedError {}
