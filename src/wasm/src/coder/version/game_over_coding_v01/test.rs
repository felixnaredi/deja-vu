#[cfg(test)]
mod temp
{
  use crate::rng::{
    IndexedPermutation,
    Konadare192PxPlusPlus,
    KSINK,
  };
  #[test]
  fn f()
  {
    // 2123399681707578173
    // 17792727522076118826
    // 3419954961163653668
    println!(
      "{}",
      KSINK::permute_index(3419954961163653668, 17792727522076118826)
    );

    panic!("REMOVE THIS TEST BEFORE PR")
  }

  #[test]
  fn g()
  {
    let mut rng = Konadare192PxPlusPlus::from_seed(8366408846780968924);
    let mut s: Vec<Option<usize>> = (0..3)
      .map(|_| rng.next_with_upper_bound(64) as usize)
      .map(|x| Some(x))
      .collect();
    s.sort();
    println!("{:?}", s);

    // [Some(30), Some(31), Some(54)]
    panic!("REMOVE THIS TEST BEFORE PR")
  }
}

use std::iter;

use super::GameOverCodingV01;
use crate::{
  coder::{
    unseen_set_id::UnseenSetID,
    version::game_over_coding_v01::GameOverCodingV01Error,
  },
  game_over::GameOver,
};

#[test]
fn encode_decode_equals_id()
{
  let unseen: Vec<[u8; 1]> = (0..64).map(|x| [x]).collect();
  let game_over = GameOver::new(
    9940370477626720397,
    UnseenSetID::Unspecified,
    unseen.clone(),
    0.4.try_into().unwrap(),
    [Some(9), Some(15), Some(35)],
  );
  let encoded = GameOverCodingV01::encode(game_over.clone(), UnseenSetID::Unspecified).unwrap();
  let decoded = GameOverCodingV01::decode(encoded.try_into().unwrap(), unseen).unwrap();

  assert_eq!(decoded.element_checksum(), game_over.element_checksum());
  assert_eq!(decoded.score(), game_over.score());
  assert_eq!(decoded.lives(), game_over.lives());
  assert!(iter::zip(decoded.into_iter(), game_over.into_iter()).all(|(x, y)| x == y));
}

#[test]
fn detect_use_of_wrong_set_when_encoding()
{
  let game_over = GameOver::new(
    13206503794884972104,
    UnseenSetID::Unspecified,
    (0..64).map(|x| [x]).collect(),
    0.4.try_into().unwrap(),
    [Some(4), Some(33), Some(45)],
  );
  assert!(matches!(
    GameOverCodingV01::encode(game_over.clone(), UnseenSetID::Unspecified,),
    Err(GameOverCodingV01Error::BadElementChecksum)
  ));
}

#[test]
fn detect_use_of_wrong_set_when_decoding()
{
  let game_over = GameOver::new(
    622451429113938556,
    UnseenSetID::Unspecified,
    (0..64).map(|x| [x]).collect(),
    0.4.try_into().unwrap(),
    [Some(30), Some(31), Some(54)],
  );
  let encoded = GameOverCodingV01::encode(game_over.clone(), UnseenSetID::Unspecified).unwrap();
  assert!(matches!(
    GameOverCodingV01::decode(
      encoded.try_into().unwrap(),
      (64..128).map(|x| [x]).collect(),
    ),
    Err(GameOverCodingV01Error::BadElementChecksum)
  ));
}

#[test]
fn decode_detect_bad_element_checksum()
{
  panic!("not implemented");
}
