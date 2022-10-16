use super::constant;

pub trait IndexedPermutation
{
  fn permute_index(x: u64, c: u64) -> u64;

  fn stir(s: &mut [u64], dissallow_all_zeros: bool)
  {
    match s.len() {
      0 => return,
      1 => {
        s[0] = Self::permute_index(s[0], constant::SQRT19);
        if dissallow_all_zeros && s[0] == 0 {
          s[0] = Self::permute_index(0, constant::SQRT5);
        }
      }
      n => {
        for _ in 0..2 {
          if n & (n - 1) != 0 {
            for i in 0..n {
              s[i] = Self::permute_index(s[i], s[(i + n - 1) % n])
            }
          } else {
            for i in 0..n {
              s[i] = Self::permute_index(s[i], s[(i + n - 1 & (n - 1))]);
            }
          }
          if dissallow_all_zeros && s.iter().all(|x| x == &0) {
            s[0] = constant::SQRT5;
          }
        }
      }
    }
  }

  fn mix(s: &mut [u64], seed: u64)
  {
    for (i, x) in s.iter_mut().enumerate() {
      *x = seed + i as u64;
    }
    Self::stir(s, true);
  }

  fn hash(seed: u64, bytes: &[u8]) -> u64
  {
    bytes
      .iter()
      .fold(seed, |c, &x| Self::permute_index(x as u64, c))
  }
}

// -------------------------------------------------------------------------------------------------
// KNOMUL
// -------------------------------------------------------------------------------------------------

pub struct KNOMUL;

impl IndexedPermutation for KNOMUL
{
  fn permute_index(mut x: u64, mut c: u64) -> u64
  {
    for i in 0..5 {
      x ^= x.rotate_right(25) ^ x.rotate_right(49);
      c += constant::SQRT3 + (c << 15) + (c << 7) + i;
      c ^= c >> 47 ^ c >> 23;
      x += c;
      x ^= x >> 11 ^ x >> 3;
    }
    x
  }
}

// -------------------------------------------------------------------------------------------------
// KSINK
// -------------------------------------------------------------------------------------------------

pub struct KSINK;

impl IndexedPermutation for KSINK
{
  fn permute_index(mut x: u64, mut c: u64) -> u64
  {
    for _ in 0..3 {
      c += constant::SQRT3;
      c ^= c.rotate_right(49) ^ c.rotate_right(25);
      x ^= (x >> 47) ^ (x >> 29);
      x += c;
      c *= constant::SQRT5;
      x *= constant::SQRT19;
    }
    x
  }
}
