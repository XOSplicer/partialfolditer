extern crate partialfolditer;

use std::collections::VecDeque;

use partialfolditer::partialfolditer::PartialFoldIterator;
use partialfolditer::lastniter::LastNIter;

fn main() {
  let multiply_int = (1_i32..5).partial_fold(1, |&acc, &x| acc * x).collect::<Vec<_>>();
  println!("multiply integers: {:?}", multiply_int);

  let fold_to_vec = (0_i32..5)
      .partial_fold(Vec::<i32>::new(), |ref acc, &x| {
        let mut a = acc.to_vec();
        a.push(x);
        a
      })
      .collect::<Vec<_>>();
  println!("fold to vector: {:?}", fold_to_vec);

  let sum_last_5 = (0_i32..)
      .last_n(5)
      .take(7)
      .map(|v: VecDeque<i32>| v.iter().sum())
      .collect::<Vec<i32>>();
  println!("sum of last 5 numbers: {:?}", sum_last_5);

}
