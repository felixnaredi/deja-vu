
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
