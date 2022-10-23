use super::*;
use crate::{
  coder::{
    UnseenSetID,
    Version00Coding,
  },
  game::SeenThreshold,
  game_over::GameOver,
  rng::{
    IndexedPermutation,
    Konadare192PxPlusPlus,
    KSINK,
  },
};

const S: u64 = 5943847430032560006;

fn generate_game_over(
  s: u64,
  unseen_set_id: Option<UnseenSetID>,
  seen_threshold: Option<SeenThreshold>,
) -> (GameOver<String>, Vec<String>)
{
  let mut rng = Konadare192PxPlusPlus::from_seed(s);

  let mut incorrect = [
    Some(rng.next_with_upper_bound(256) as usize),
    Some(rng.next_with_upper_bound(256) as usize),
    Some(rng.next_with_upper_bound(256) as usize),
  ];
  incorrect.sort();

  let seed = rng.next();

  let unseen: Vec<String> = (0..(256 + rng.next_with_upper_bound(64)))
    .map(|_| {
      base64::encode(
        (0..(8 + rng.next_with_upper_bound(32)))
          .map(|_| rng.next_with_upper_bound(256).try_into().unwrap())
          .collect::<Vec<u8>>()
          .as_slice(),
      )
    })
    .collect();

  (
    GameOver::new(
      seed,
      unseen_set_id.unwrap_or(UnseenSetID::Unspecified),
      unseen.clone(),
      seen_threshold.unwrap_or_else(|| {
        (rng.next() as f64 / ((1u64 << 63) | ((1u64 << 63) - 1)) as f64)
          .try_into()
          .unwrap()
      }),
      incorrect,
    ),
    unseen,
  )
}

#[test]
fn f()
{
  println!(
    "{}",
    KSINK::hash(S, "2202 TSEC 85:62:71 32 tcO nuS".as_bytes())
  )
}

#[cfg(test)]
mod version00coding
{
  use super::*;

  #[test]
  fn encode_decode_same_as_id()
  {
    let (game_over, unseen) = generate_game_over(
      542338303675782954,
      Some(UnseenSetID::DictionaryFr01),
      Some(0.4.try_into().unwrap()),
    );
    let encoded = SealedEncodedGameOver::new::<Version00Coding, _>(&game_over).unwrap();
    let decoded: GameOver<String> = (encoded, unseen).try_into().unwrap();
    assert_eq!(decoded, game_over);
  }

  #[test]
  fn encode_outputs_expected_literal()
  {
    let (game_over, _) = generate_game_over(
      10607240176095180561,
      Some(UnseenSetID::DictionaryFr01),
      Some(0.4.try_into().unwrap()),
    );

    assert_eq!(serde_urlencoded::to_string(
      SealedEncodedGameOver::new::<Version00Coding, _>(&game_over).unwrap()
    )
    .unwrap(),
    "version=00&checksum=15367392902022108743&data=eyJ1bnNlZW5faWQiOiJEaWN0aW9uYXJ5RnIwMSIsInNlZWQiOjc3NzAzNzYyMDA2OTQ2Mjc5MTcsImluY29ycmVjdF9jb21taXRzIjpbMzMsNDksMTIxXX0%3D&unseen_set_id=DictionaryFr01")
  }

  #[test]
  fn decode_literal_outputs_expected_game_over()
  {
    let s = "version=00&checksum=15965453229032169769&data=eyJ1bnNlZW5faWQiOiJEaWN0aW9uYXJ5RnIwMSIsInNlZWQiOjk3NTIwMTE1MzkzNzAxMjE1MDgsImluY29ycmVjdF9jb21taXRzIjpbMTEsMTI2LDI0OV19&unseen_set_id=DictionaryFr01";

    let (game_over, unseen) = generate_game_over(
      12837024807937892714,
      Some(UnseenSetID::DictionaryFr01),
      Some(0.4.try_into().unwrap()),
    );

    let decoded: GameOver<String> = (serde_urlencoded::from_str(s).unwrap(), unseen)
      .try_into()
      .unwrap();
    assert_eq!(decoded, game_over);
  }

  #[test]
  fn decode_literal_with_corrupt_checksum()
  {
    let (_, unseen) = generate_game_over(
      9354512038846019256,
      Some(UnseenSetID::DictionaryFr01),
      Some(0.4.try_into().unwrap()),
    );

    let s = "version=00&checksum=114325682255183593&data=eyJ1bnNlZW5faWQiOiJEaWN0aW9uYXJ5RnIwMSIsInNlZWQiOjE2MTM3Mjg1Nzg3MTIwODk2OTc0LCJpbmNvcnJlY3RfY29tbWl0cyI6Wzc2LDc3LDE2MF19&unseen_set_id=DictionaryFr01";
    assert!(matches!(
      TryInto::<GameOver<String>>::try_into((
        serde_urlencoded::from_str::<SealedEncodedGameOver>(s).unwrap(),
        unseen
      )),
      Err(_)
    ));
  }

  #[test]
  fn decode_literal_with_corrupt_data()
  {
    let (_, unseen) = generate_game_over(
      3088894222515458658,
      Some(UnseenSetID::DictionaryFr01),
      Some(0.4.try_into().unwrap()),
    );

    let s = "version=00&checksum=15649906318563636177&data=eyJ1bnNlZW5faWQiOiJEaWN0aW9uYXJ5RnIwMSIsInNlZWQiOjE5MzUyNTQ2NDY0MTkzOTI1NzYsImluY29ycmVjdF9jb21taXRzIjpbMTQsMTUsMTUzXX0%3D&unseen_set_id=DictionaryFr01";
    assert!(matches!(
      TryInto::<GameOver<String>>::try_into((
        serde_urlencoded::from_str::<SealedEncodedGameOver>(s).unwrap(),
        unseen
      )),
      Err(_)
    ));
  }

  #[test]
  fn decode_url_with_missing_fields()
  {
    let (game_over, unseen) = generate_game_over(
      17561082018945952039,
      Some(UnseenSetID::DictionaryFr01),
      Some(0.4.try_into().unwrap()),
    );

    let version = "version=00";
    let checksum = "checksum=16916324704992223141";
    let data = "data=eyJ1bnNlZW5faWQiOiJEaWN0aW9uYXJ5RnIwMSIsInNlZWQiOjcxMzA5MDM0Nzg0NDc2NzAwMjUsImluY29ycmVjdF9jb21taXRzIjpbMTMxLDE3NiwyNDFdfQ%3D%3D";
    let unseen_set_id = "unseen_set_id=DictionaryFr01";

    for i in 0..15 {
      let fields: Vec<String> = [
        (i & 1 != 0).then(|| version),
        (i & 2 != 0).then(|| checksum),
        (i & 4 != 0).then(|| data),
        (i & 8 != 0).then(|| unseen_set_id),
      ]
      .iter()
      .filter_map(|x| x.map(|x| x.into()))
      .collect();
      let fields = fields.join("&");

      if i == 7 {
        // All fields but `unseen_set_id` is present. This is a valid URL.
        let decoded: GameOver<String> =
          (serde_urlencoded::from_str(&fields).unwrap(), unseen.clone())
            .try_into()
            .unwrap();
        assert_eq!(decoded, game_over);
      } else {
        // Invalid URL. This will be detected when deserializing the URL into a
        // `SealedEncodedGameOver`
        assert!(matches!(
          serde_urlencoded::from_str::<SealedEncodedGameOver>(&fields),
          Err(_)
        ));
      }
    }
  }
}

/*
mod game_over_coder_v01
{
  use std::iter;

  use crate::{
    coder::{
      GameOverCoderV01,
      SealedEncodedGameOver,
      UnseenSetID,
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
    let encoded = SealedEncodedGameOver::new::<GameOverCoderV01, _>(game_over.clone()).unwrap();
    let decoded = GameOver::try_from((encoded, unseen)).unwrap();

    assert_eq!(decoded.element_checksum(), game_over.element_checksum());
    assert_eq!(decoded.score(), game_over.score());
    assert_eq!(decoded.lives(), game_over.lives());
    assert!(iter::zip(decoded.into_iter(), game_over.into_iter()).all(|(x, y)| x == y));
  }

  #[test]
  fn detect_use_of_wrong_set_when_decoding()
  {
    let e = SealedEncodedGameOver::new::<GameOverCoderV01, _>(GameOver::new(
      622451429113938556,
      UnseenSetID::Unspecified,
      // The unseen set used when encoding...
      (0..64).map(|x| [x]).collect(),
      0.4.try_into().unwrap(),
      [Some(30), Some(31), Some(54)],
    ))
    .unwrap();

    assert!(GameOver::try_from((e, (64..128).map(|x| [x]).collect())).is_err());
  }
}
*/
