use serde::{
  Deserialize,
  Serialize,
};

#[derive(Builder, Debug, Serialize, Deserialize)]
pub struct Encoded
{
  version: String,
  checksum: u64,
  data: String,
}

impl Encoded
{
  /// Identifier of the coder version used.
  pub fn version(&self) -> &str
  {
    self.version.as_ref()
  }

  /// Version specific checksum of `data`.
  pub fn checksum(&self) -> u64
  {
    self.checksum
  }

  /// Base64 encoded data.
  pub fn data(&self) -> &str
  {
    self.data.as_ref()
  }
}
