extern crate rand;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::env;

struct Package {
  cpu : u8,
  mem : u8,
  net : u8,
}

type Valuation = Vec<(Package, Option<u32>)>;

// Use definition of AgentType from Parkes and Singh, 2003
// "An MDP-Based Approach to Online Mechanism Design"
struct AgentType {
  arrival   : u16,
  departure : u16,
  valuation : Valuation
}

fn generate_random_agent_type(rng : &mut StdRng) -> AgentType {
  // TODO: implement
  return AgentType{arrival : 0, departure : 0, valuation : Valuation::new()};
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
