use super::knomul::KNOMUL;

const KONADARE192_INC: u64 = 0xBB67AE8584CAA73B;

#[derive(Debug)]
pub struct Konadare192PxPlusPlus
{
  a: u64,
  b: u64,
  c: u64,
}

impl Konadare192PxPlusPlus
{
  /// Creates a new `Konadare192PxPlusPlus` from `seed`.
  pub fn from_seed(seed: u64) -> Konadare192PxPlusPlus
  {
    let mut ss = [0; 3];
    KNOMUL::mix(&mut ss, seed);

    Konadare192PxPlusPlus {
      a: ss[0],
      b: ss[1],
      c: ss[2],
    }
  }

  /// Next pseudo random number.
  pub fn next(&mut self) -> u64
  {
    let out = self.b ^ self.c;
    let a = self.a ^ self.a >> 32;
    self.a += KONADARE192_INC;
    self.b = (self.b + a).rotate_right(11);
    self.c = (self.c + self.b).rotate_right(56);

    out
  }

  /// Generates a pseodo random number in the range [0, `upper_bound`).
  ///
  /// From Daniel Lemire's "Fast Random Integer Generation in an Interval",
  /// https://arxiv.org/pdf/1805.10941.pdf
  ///
  /// The version below was written by Pelle Evensen.
  pub fn next_with_upper_bound(&mut self, upper_bound: u32) -> u32
  {
    let mut x = (self.next() & ((1 << 31) - 1)) as u32;
    let mut m = x as u64 * ((upper_bound as u64) << 1);
    let mut l = x * upper_bound;
    if l < upper_bound {
      let t = (!upper_bound + 1) % upper_bound;
      while l < t {
        x = (self.next() & ((1 << 31) - 1)) as u32;
        m = x as u64 * ((upper_bound as u64) << 1);
        l = x * upper_bound;
      }
    }
    (m >> 32) as u32
  }
}
