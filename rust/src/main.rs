extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::time::SystemTime;

fn main() {
  const MIN: i32 = 1; 
  const MAX: i32 = 100_001; 
  const N: usize = 1_000_000;
  const IS_PRINT: bool = false;
  
  let numbers = get_rand_numbers(MIN, MAX, N);
  
  let time_start = SystemTime::now();
  let sorted = quicksort(&numbers);
  let time_elapsed = time_start.elapsed().unwrap();
  println!("Time used: {}.{} seconds", time_elapsed.as_secs(), time_elapsed.subsec_nanos());

  if IS_PRINT {
    println!("Random: {:?}", numbers); // debug formatter
    println!("Sorted: {:?}", sorted); // debug formatter
  }
}

// get an arry of random numbers within a range
fn get_rand_numbers(min: i32, max: i32, n: usize) -> Vec<i32> {
  let mut nums = vec![];
  
  for _ in 0..n {
   let num = rand::thread_rng().gen_range(min, max);
    nums.push(num);
  }
  
  nums
}

// quicksort
fn quicksort(nums: &[i32]) -> Vec<i32> {
  if nums.len() == 1 {
    return vec![nums[0]];
  }
  
  let cmp_num = nums[0];
  
  let mut less  = Vec::new();
  let mut equal = Vec::new();
  let mut more  = Vec::new();
  for i in nums.iter() {
    match i.cmp(&cmp_num) {
      Ordering::Less    => less.push(*i),
      Ordering::Equal   => equal.push(*i),
      Ordering::Greater => more.push(*i),
    }
  }
  
  let mut sorted = vec![];
  
  if less.len() > 0 {
    sorted.extend(quicksort(&less));
  }
  
  sorted.extend(equal);
  
  if more.len() > 0 {
    sorted.extend(quicksort(&more));
  }
  
  sorted
}