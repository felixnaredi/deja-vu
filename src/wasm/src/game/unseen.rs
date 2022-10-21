use std::{
  collections::HashMap,
  rc::Rc,
};

use crate::rng::Konadare192PxPlusPlus;

#[derive(Debug)]
pub struct Unseen<T>
{
  data: Rc<Vec<T>>,
  size: usize,
  indices: HashMap<usize, usize>,
}

// Implementing clone manually removes requirement for `T` to implement `Clone`.
impl<T> Clone for Unseen<T>
{
  fn clone(&self) -> Self
  {
    Self {
      data: Rc::clone(&self.data),
      size: self.size.clone(),
      indices: self.indices.clone(),
    }
  }
}

impl<T> Unseen<T>
{
  /// Creates a new `Unseen`.
  pub fn new(data: Vec<T>) -> Unseen<T>
  {
    let size = data.len();
    Unseen {
      data: Rc::new(data),
      size,
      indices: HashMap::new(),
    }
  }

  /// Polls a random element from the data. The element at the polled index will never be polled
  /// twice.
  pub fn poll(&mut self, rng: &mut Konadare192PxPlusPlus) -> Option<&T>
  {
    if self.size == 0 {
      None
    } else {
      let i = rng.next_with_upper_bound(self.size as u32) as usize;
      let mut j = i;

      while let Some(&k) = self.indices.get(&j) {
        j = k;
      }
      self.size -= 1;
      self.indices.insert(i, self.size);
      self.data.get(j)
    }
  }

  pub fn reset(&mut self)
  {
    self.size = self.data.len();
    self.indices = HashMap::new();
  }
}

#[cfg(test)]
mod test
{
  use std::collections::HashSet;

  use super::*;
  use crate::rng::Konadare192PxPlusPlus;

  #[test]
  fn never_poll_same_element_twice()
  {
    let mut unseen = Unseen::new((0..8).collect());
    let mut rng = Konadare192PxPlusPlus::from_seed(8512851105331467714);
    let mut s = HashSet::new();

    for _ in 0..8 {
      assert!(s.insert(unseen.poll(&mut rng).unwrap().clone()));
    }
  }

  #[test]
  fn none_when_empty()
  {
    let mut rng = Konadare192PxPlusPlus::from_seed(12783995747563634368);

    let mut unseen = Unseen::new(Vec::<usize>::new());
    assert_eq!(unseen.poll(&mut rng), None);

    let mut unseen = Unseen::new((0..8).collect());
    let mut i = 0;
    while unseen.poll(&mut rng).is_some() {
      i += 1;
      if i > 8 {
        panic!("`i` is greater than expected");
      }
    }
    assert_eq!(i, 8)
  }

  #[test]
  fn same_order_after_reset()
  {
    let mut rng = Konadare192PxPlusPlus::from_seed(3654338965601285362);
    let mut unseen = Unseen::new((0..8).collect());
    let mut s = Vec::new();
    while let Some(&x) = unseen.poll(&mut rng) {
      s.push(x);
    }

    let mut rng = Konadare192PxPlusPlus::from_seed(3654338965601285362);
    unseen.reset();
    for x in s.iter() {
      assert_eq!(x, unseen.poll(&mut rng).unwrap());
    }
  }

  #[test]
  fn clone_generates_same_output()
  {
    let mut rng = Konadare192PxPlusPlus::from_seed(16667478474295396221);
    let mut unseen0 = Unseen::new((0..16).collect());

    unseen0.poll(&mut rng);
    unseen0.poll(&mut rng);

    let mut rng0 = Konadare192PxPlusPlus::from_seed(12150609935105388669);
    let mut rng1 = Konadare192PxPlusPlus::from_seed(12150609935105388669);
    let mut unseen1 = unseen0.clone();

    while let (Some(x0), Some(x1)) = (unseen0.poll(&mut rng0), unseen1.poll(&mut rng1)) {
      assert_eq!(x0, x1);
    }
  }
}
