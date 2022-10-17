#[cfg(test)]
mod test;

use std::fmt::Display;

use serde::{
  Deserialize,
  Serialize,
};

use crate::{
  coder::Version00Coding,
  rng::{
    IndexedPermutation,
    KNOMUL,
  },
};

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
