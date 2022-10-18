#[cfg(test)]
mod test;

use std::fmt::Display;

use serde::{
  Deserialize,
  Serialize,
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
}

// -------------------------------------------------------------------------------------------------
// SealedEncoded
// -------------------------------------------------------------------------------------------------

/// The container of data and meta data for an `Encoded`. To access the underlying data it can be
/// cast into an `Encoded` with `SealedEncoded::try_into()`.
#[derive(Builder, Debug, Serialize, Deserialize)]
pub struct SealedEncodedGameOver
{
  version: String,
  checksum: u64,
  data: String,
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
    })
  }
}

impl TryFrom<SealedEncodedGameOver> for EncodedGameOver
{
  type Error = SealedEncodedError;

  fn try_from(s: SealedEncodedGameOver) -> Result<EncodedGameOver, SealedEncodedError>
  {
    use SealedEncodedError::*;

    if s.version != Version00Coding::id() {
      Err(UnrecognisedVersion(s.version))
    } else if s.checksum != KNOMUL::hash(Version00Coding::hash_seed(), s.data.as_bytes()) {
      Err(InvalidChecksum)
    } else {
      Ok(EncodedGameOver(s))
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
        Ok(GameOverCoderV01::decode(s.data, unseen)?)
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
