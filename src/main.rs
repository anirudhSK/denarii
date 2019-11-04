extern crate rand;
mod algorithms;
mod gurobi;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::env;
fn main() {
  let args : Vec<String> = env::args().collect();
  let seed : u64 = args[1].parse::<u64>().unwrap();
  let num_resources : u64 = args[2].parse::<u64>().unwrap();
  let mut rng : StdRng = SeedableRng::seed_from_u64(seed);
  for x in 0..100 {
    // Generate random demand vectors
    let mut vec : Vec<i16> = Vec::new();
    for _y in 0..num_resources {
      vec.push(rng.gen_range(0, 1000));
    }
    println!("Hello, world {}, random vector: {:?}!", x, vec);
  }
}
