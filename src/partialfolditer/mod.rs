use std::fmt;

#[derive(Clone)]
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct PartialFold<I, Acc, F> {
  iter: I,
  current_acc: Acc,
  f: F
}

impl<I: fmt::Debug, Acc: fmt::Debug, F> fmt::Debug for PartialFold<I, Acc, F> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.debug_struct("PartialFold")
        .field("iter", &self.iter)
        .field("current_acc", &self.current_acc)
        .finish()
  }
}

impl<I, Acc, F> Iterator for PartialFold<I, Acc, F>
  where I: Iterator + Sized,
        Acc: Clone,
        F: FnMut(&Acc, &I::Item) -> Acc {
  type Item = (Acc, I::Item);

  fn next(&mut self) -> Option<Self::Item> {
    if let Some(x) = self.iter.next() {
      self.current_acc = (self.f)(&self.current_acc, &x);
      return Some((self.current_acc.clone(), x));
    }
    None
  }

  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    self.iter.size_hint()
  }
}

pub trait PartialFoldIterator: Iterator {
  fn partial_fold<Acc, F>(self, init: Acc, f: F) -> PartialFold<Self, Acc, F>
    where Self: Sized,
          Acc: Clone,
          F: FnMut(&Acc, &Self::Item) -> Acc {
    PartialFold {
      iter: self,
      current_acc: init,
      f: f,
    }
  }
}

impl<I: Iterator> PartialFoldIterator for I {}

#[cfg(test)]
mod tests {
  use partialfolditer::PartialFoldIterator;

  #[test]
  fn test_int_sum() {
    let expected = vec![(0, 0), (1, 1), (3, 2), (6, 3), (10, 4)];
    let output = (0_i32..5).partial_fold(0, |&acc, &x| acc + x).collect::<Vec<_>>();
    assert_eq!(expected, output);
  }

  #[test]
  fn test_int_prod() {
    let expected = vec![(1, 1), (2, 2), (6, 3), (24, 4)];
    let output = (1_i32..5).partial_fold(1, |&acc, &x| acc * x).collect::<Vec<_>>();
    assert_eq!(expected, output);
  }

  #[test]
  fn test_vec_grow() {
    let expected = vec![(vec![0], 0),
                        (vec![0, 1], 1),
                        (vec![0, 1, 2], 2),
                        (vec![0, 1, 2, 3], 3),
                        (vec![0, 1, 2, 3, 4], 4)];
    let output = (0_i32..5)
        .partial_fold(Vec::<i32>::new(),|ref acc, &x| { let mut a = acc.to_vec(); a.push(x); a})
        .collect::<Vec<_>>();
    assert_eq!(expected, output);
  }

}
