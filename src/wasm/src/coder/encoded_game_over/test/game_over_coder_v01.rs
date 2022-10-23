use super::*;

#[test]
fn encode_decode_same_as_id()
{
  let (game_over, unseen) = generate_game_over(9239737542598549709, None, None);
  let encoded = SealedEncodedGameOver::new::<GameOverCoderV01, _>(&game_over).unwrap();
  let decoded: GameOver<String> = (encoded, unseen).try_into().unwrap();
  assert_eq!(decoded, game_over);
}

#[test]
fn encode_outputs_expected_literal()
{
  let (game_over, _) = generate_game_over(12504334248584760776, None, None);
  assert_eq!(serde_urlencoded::to_string(
     SealedEncodedGameOver::new::<GameOverCoderV01, _>(&game_over).unwrap()
   )
   .unwrap(),
   "version=goc-v01&checksum=6196960203795175085&data=eyJzZWVkIjoyMDg5MDM0NDI0NzI0NTg3NDI4LCJzZWVuX3RocmVzaG9sZCI6NzQ0NTY5Nzg4LCJpbmNvcnJlY3RfY29tbWl0cyI6WzEwMCwyMTYsMjI5XSwiZWxlbWVudF9jaGVja3N1bSI6NjYzODY2NTM4MjQzMzEyNDAwMn0%3D&unseen_set_id=Unspecified"
   );
}

#[test]
fn decode_literal_outputs_expected_game_over()
{
  let s = "version=goc-v01&checksum=9581572683804129452&data=eyJzZWVkIjoxODI0NzEyNTcwNzA3Mjk1MTMxLCJzZWVuX3RocmVzaG9sZCI6NDU0MjQwOTI3LCJpbmNvcnJlY3RfY29tbWl0cyI6WzEyNiwxODYsMTk1XSwiZWxlbWVudF9jaGVja3N1bSI6MTU2MTYzNDEyNzMzOTc2OTgzMzh9&unseen_set_id=Unspecified";
  let (game_over, unseen) = generate_game_over(5342632399507448204, None, None);

  let decoded: GameOver<String> = (serde_urlencoded::from_str(s).unwrap(), unseen)
    .try_into()
    .unwrap();
  assert_eq!(decoded, game_over);
}

#[test]
fn decode_literal_with_corrupt_checksum()
{
  let (_, unseen) = generate_game_over(4405435083803308078, None, None);
  let s = "version=goc-v01&checksum=3417369460210627685&data=eyJzZWVkIjoxMTQ2MjY4NDcyNTcyMDU1MTM2Nywic2Vlbl90aHJlc2hvbGQiOjE3MzYzMDgxOSwiaW5jb3JyZWN0X2NvbW1pdHMiOlsxLDg1LDIwMV0sImVsZW1lbnRfY2hlY2tzdW0iOjk1NjE2Mzk2OTIzMDU4NzMxMDV9&unseen_set_id=Unspecified";
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
  let (_, unseen) = generate_game_over(16739628093234127117, None, None);
  let s = "version=goc-v01&checksum=5597641129812305779&data=eyJzZWVkIjoxMjI3OTE5OTkyMTE2MjkyNjA2MCwic2Vlbl90aHJlc2hvbGQiOjQ0OTc2MTMyMywiaW5jb3JyZWN0X2NvbW1pdHMiOls4OSwyMDEsMjU0XSwiZWxlbWVudF9jaGVja3N1bSI6MTgzNDI4MzQyNzM0MDA4OTYxMjl9&unseen_set_id=Unspecified";
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
  let (_, unseen) = generate_game_over(10012246512391581010, None, None);
  let version = "version=goc-v01";
  let checksum = "checksum=13561387729840425778";
  let data = "data=eyJzZWVkIjoxNTc2NDEzMzM4NzA1NzY2MzQwLCJzZWVuX3RocmVzaG9sZCI6NTYzODM1ODk4LCJpbmNvcnJlY3RfY29tbWl0cyI6WzIyLDIzLDg2XSwiZWxlbWVudF9jaGVja3N1bSI6MTgyNzI1NzMwNzg1Njk3Nzk3MTJ9";
  let unseen_set_id = "unseen_set_id=Unspecified";

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
      // All fields but `unseen_set_id` is present. This will not be detected when deserializing the
      // URL into a `SealedEncodedGameOver` but when decoding into a `GameOver`.
      assert!(matches!(
        TryInto::<GameOver<String>>::try_into((
          serde_urlencoded::from_str::<SealedEncodedGameOver>(&fields).unwrap(),
          unseen.clone()
        )),
        Err(_)
      ));
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

#[test]
fn decode_url_with_wrong_unseen_set_id()
{
  let (_, unseen) = generate_game_over(
    10030715364591628463,
    Some(UnseenSetID::Top999WiktionaryFr),
    None,
  );
  let s = "version=goc-v01&checksum=1487839826841008537&data=eyJzZWVkIjo5ODE2MDU4Mzg1MzIzNzUxMjMzLCJzZWVuX3RocmVzaG9sZCI6NDE3MTk1NzkzLCJpbmNvcnJlY3RfY29tbWl0cyI6WzE1MSwyMTMsMjU0XSwiZWxlbWVudF9jaGVja3N1bSI6MzMyMTQwODUzNDE4MzM0NzQwOH0%3D&unseen_set_id=DictionaryFr01";
  assert!(matches!(
    TryInto::<GameOver<String>>::try_into((
      serde_urlencoded::from_str::<SealedEncodedGameOver>(&s).unwrap(),
      unseen.clone()
    )),
    Err(_)
  ));
}

#[test]
fn decode_url_with_modified_unseen()
{
  let (_, unseen) = generate_game_over(2657328141259618373, None, None);
  let s = "version=goc-v01&checksum=5106451020992070949&data=eyJzZWVkIjoxODMwMDExNjgxODczODIzODI5LCJzZWVuX3RocmVzaG9sZCI6NDAwNTAxMDM1LCJpbmNvcnJlY3RfY29tbWl0cyI6Wzc0LDE4MiwyMjNdLCJlbGVtZW50X2NoZWNrc3VtIjo5MzY4NTg2ODYzNjQxMDM0NTk2fQ%3D%3D&unseen_set_id=Unspecified";
  assert!(matches!(
    TryInto::<GameOver<String>>::try_into((
      serde_urlencoded::from_str::<SealedEncodedGameOver>(&s).unwrap(),
      unseen.into_iter().rev().collect()
    )),
    Err(_)
  ));
}
