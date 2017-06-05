extern crate partialfolditer;
use partialfolditer::PartialFoldIterator;

fn main() {
    let mult_int = (1_i32..5).partial_fold(1, |&acc, &x| acc * x).collect::<Vec<_>>();
    println!("multiply integers: {:?}", mult_int);
    
    let fold_to_vec = (0_i32..5)
        .partial_fold(Vec::<i32>::new(),|ref acc, &x| { let mut a = acc.to_vec(); a.push(x); a})
        .collect::<Vec<_>>();
    println!("fold to vector: {:?}", fold_to_vec);
}
