use rand::{thread_rng, Rng};
use std::vec::Vec;
use std::time::Instant;

fn quicksort_parallel<T: Ord + Send>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    }
    
    let pivot_index = partition(slice);
    
    let (left, right) = slice.split_at_mut(pivot_index);
    let (_, right) = right.split_at_mut(1);
    
    rayon::join(|| quicksort_parallel(left), || quicksort_parallel(right));
}

fn partition<T: Ord>(slice: &mut [T]) -> usize {
    let len = slice.len();
    let pivot_index = rand::thread_rng().gen_range(0..len);
    slice.swap(pivot_index, len - 1);
    
    let mut i = 0;
    for j in 0..len - 1 {
        if slice[j] <= slice[len - 1] {
            slice.swap(i, j);
            i += 1;
        }
    }
    
    slice.swap(i, len - 1);
    i
}

fn quicksort_sequential<T: Ord>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    }
    
    let pivot_index = partition(slice);
    let (left, right) = slice.split_at_mut(pivot_index);
    let (_, right) = right.split_at_mut(1);
    
    quicksort_sequential(left);
    quicksort_sequential(right);
}

fn main() {
    let mut rng = thread_rng();
    
    let n = 1_000_000;
    println!("Number of elements: {}", n);
    
    let mut a : Vec<i64> = (0..n).map(|_| rng.gen_range(1..1_000_000_000)).collect();
    let mut b = a.clone();

    let start_iterative = Instant::now();
    quicksort_sequential(a.as_mut_slice());
    println!("Sequntial consumed time ms: {}", start_iterative.elapsed().as_millis());
    
    let start_parallel = Instant::now();
    quicksort_parallel(b.as_mut_slice());
    println!("Paralelized consumed time ms: {}", start_parallel.elapsed().as_millis());
    
}