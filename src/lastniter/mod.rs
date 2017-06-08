use std::fmt;
use std::collections::VecDeque;

#[derive(Clone)]
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct LastN<I, T> {
  iter: I,
  n: usize,
  last_n: VecDeque<T>
}

impl<I: fmt::Debug, T: fmt::Debug> fmt::Debug for LastN<I, T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.debug_struct("LastN")
        .field("iter", &self.iter)
        .field("last_n", &self.last_n)
        .finish()
  }
}

impl<I> Iterator for LastN<I, I::Item>
  where I: Iterator + Sized,
        I::Item: Clone{
  type Item = VecDeque<I::Item>;

  fn next(&mut self) -> Option<Self::Item> {
    if let Some(x) = self.iter.next() {
      if self.last_n.len() >= self.n {
        let _ = self.last_n.pop_front();
      }
      self.last_n.push_back(x);
      return Some(self.last_n.clone());
    }
    None
  }

  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    self.iter.size_hint()
  }
}

pub trait LastNIter: Iterator {
  fn last_n(self, n: usize) -> LastN<Self, Self::Item>
    where Self: Sized {
    LastN {
      iter: self,
      n: n,
      last_n: VecDeque::<Self::Item>::with_capacity(n),
    }
  }
}

impl<I: Iterator> LastNIter for I {}


#[cfg(test)]
mod tests {
  use lastniter::LastNIter;
  use std::collections::VecDeque;

  #[test]
  fn test_int() {
    let expected = vec![VecDeque::from(vec![0]),
                        VecDeque::from(vec![0, 1]),
                        VecDeque::from(vec![0, 1, 2]),
                        VecDeque::from(vec![1, 2, 3])];
    let output = (0..)
                  .last_n(3)
                  .take(4)
                  .collect::<Vec<_>>();
    assert_eq!(expected, output);
  }

  #[test]
  fn test_sum() {
    let expected = vec![0, 1, 3, 6, 10, 15, 20];
    let output = (0_i32..)
                      .last_n(5)
                      .take(7)
                      .map(|v: VecDeque<i32>| v.iter().sum())
                      .collect::<Vec<i32>>();
    assert_eq!(expected, output);
  }

  #[test]
  fn test_mean() {
    let expected = vec![0, 1, 3, 5];
    let output = (0i32..)
                    .map(|x| 2 * x)
                    .last_n(2)
                    .take(4)
                    .map(|v: VecDeque<i32>| v.iter().sum::<i32>() / 2)
                    .collect::<Vec<i32>>();
    assert_eq!(expected, output);
  }

}