use std::fmt::Display;

use serde::{
  Deserialize,
  Serialize,
};

use crate::{
  coder::Version00Coding,
  rng::KNOMUL,
};

#[derive(Builder, Debug, Serialize, Deserialize)]
pub struct SealedEncoded
{
  version: String,
  checksum: u64,
  data: String,
}

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
      InvalidChecksum => writeln!(f, "the data has been currupted"),
      UnrecognisedVersion(s) => writeln!(f, "version '{}' is not recognised", s),
    }
  }
}

pub struct Encoded(SealedEncoded);

impl SealedEncoded
{
  pub fn valid(self) -> Result<Encoded, SealedEncodedError>
  {
    use SealedEncodedError::*;

    if self.version != Version00Coding::id() {
      Err(UnrecognisedVersion(self.version))
    } else if self.checksum != KNOMUL::hash(Version00Coding::hash_seed(), self.data.as_bytes()) {
      Err(InvalidChecksum)
    } else {
      Ok(Encoded(self))
    }
  }
}

impl Encoded
{
  /// Base64 encoded data.
  pub fn data(&self) -> &str
  {
    self.0.data.as_ref()
  }
}
