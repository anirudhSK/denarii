extern crate rand;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::env;
struct Package {
  cpu : u8,
  mem : u8,
  net : u8,
}

type Bid = Vec<(Package, Option<u32>)>;

fn generate_random_bid(&mut StdRng) -> u32 {
  let mut vec_length = 0;   // TODO: Generate random bid length
  let mut bid = Vec::new(); // TODO: Generate random bids
}

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
