#[cfg(test)]
mod game_over_coder_v01;

#[cfg(test)]
mod version00coding;

use std::collections::BTreeSet;

use super::*;
use crate::{
  coder::UnseenSetID,
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

  let unseen: BTreeSet<String> = (0..(256 + rng.next_with_upper_bound(64)))
    .map(|_| {
      base64::encode(
        (0..(8 + rng.next_with_upper_bound(32)))
          .map(|_| rng.next_with_upper_bound(256).try_into().unwrap())
          .collect::<Vec<u8>>()
          .as_slice(),
      )
    })
    .collect();
  let unseen: Vec<String> = unseen.into_iter().collect();

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
    KSINK::hash(S, "2202 TSEC 61:23:81 32 tcO nuS".as_bytes())
  )
}
