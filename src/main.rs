extern crate rand;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::env;
fn main() {
  let args : Vec<String> = env::args().collect();
  let seed : u64 = args[1].parse::<u64>().unwrap();
  let mut rng : StdRng = SeedableRng::seed_from_u64(seed);
  let mut vec : Vec<i8> = vec![1, 2, 3];
  for x in 0..10 {
    // Generate random demand vectors
    // with random bids.
    for i in 0..vec.len() {
      vec[i] = rng.gen(); 
    }
    println!("Hello, world {}, random vector: {:?}!", x, vec);
  }
}
