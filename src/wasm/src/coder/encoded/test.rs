use std::iter;

use super::*;
use crate::{
  coder::Version00Coding,
  game_over::GameOver,
};

#[test]
fn encode_decode_same_as_id()
{
  let unseen: Vec<i32> = (0..64).collect();

  let game_over = GameOver::new(
    2313308731114687875,
    unseen.clone(),
    0.4.try_into().unwrap(),
    [Some(1), Some(24), Some(30)],
  );
  let encoded = Version00Coding::encode(&game_over);
  let decoded = Version00Coding::decode(encoded.try_into().unwrap(), unseen).unwrap();

  let mut i = 31;
  for (u0, u1) in iter::zip(game_over.into_iter(), decoded.into_iter()) {
    assert_eq!(u0, u1);
    i -= 1;
  }
  assert_eq!(i, 0);
}

#[test]
fn modifying_checksum_throws_invalid_checksum_error()
{
  let unseen: Vec<i32> = (0..64).collect();

  let game_over = GameOver::new(
    2313308731114687875,
    unseen.clone(),
    0.4.try_into().unwrap(),
    [Some(16), Some(30), Some(34)],
  );
  let mut encoded = Version00Coding::encode(&game_over);

  // xor the checksum with a random value.
  encoded.checksum ^= KNOMUL::permute_index(encoded.checksum, 12228011056065030022);

  assert_eq!(
    Encoded::try_from(encoded).unwrap_err().to_string(),
    SealedEncodedError::InvalidChecksum.to_string()
  );
}

#[test]
fn modifying_data_throws_invalid_checksum_error()
{
  let unseen: Vec<i32> = (0..64).collect();

  let game_over = GameOver::new(
    2313308731114687875,
    unseen.clone(),
    0.4.try_into().unwrap(),
    [Some(21), Some(46), Some(47)],
  );
  let mut encoded = Version00Coding::encode(&game_over);

  // Swap two letters in the base64 encoded data.

  let (i, j) = (
    KNOMUL::permute_index(encoded.checksum, 15986057670448281097) as usize % encoded.data.len(),
    KNOMUL::permute_index(encoded.checksum, 15772190296526768807) as usize % encoded.data.len(),
  );
  let mut b: Vec<u8> = encoded.data.as_bytes().iter().cloned().collect();
  b.swap(i, j);
  encoded.data = String::from_utf8(b).unwrap();

  assert_eq!(
    Encoded::try_from(encoded).unwrap_err().to_string(),
    SealedEncodedError::InvalidChecksum.to_string()
  );
}

#[test]
fn invalid_version_throws_unrecognised_version_error()
{
  let unseen: Vec<i32> = (0..64).collect();

  let game_over = GameOver::new(
    2313308731114687875,
    unseen.clone(),
    0.4.try_into().unwrap(),
    [Some(22), Some(42), Some(49)],
  );
  let mut encoded = Version00Coding::encode(&game_over);
  encoded.version = "bad-version".into();

  // The checksum is modified too. This to ensure that version error is prioritized over invalid
  // checksum.
  encoded.checksum = KNOMUL::permute_index(encoded.checksum, 10041457562034272223);

  // The data is modified too. This to ensure that version error is prioritized over invalid
  // data.
  let (i, j) = (
    KNOMUL::permute_index(encoded.checksum, 6813646448085885511) as usize % encoded.data.len(),
    KNOMUL::permute_index(encoded.checksum, 16217660822986183576) as usize % encoded.data.len(),
  );
  let mut b: Vec<u8> = encoded.data.as_bytes().iter().cloned().collect();
  b.swap(i, j);
  encoded.data = String::from_utf8(b).unwrap();

  assert_eq!(
    Encoded::try_from(encoded).unwrap_err().to_string(),
    SealedEncodedError::UnrecognisedVersion("bad-version".into()).to_string()
  );
}
