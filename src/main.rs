mod algorithms;
mod gurobi;
mod simulator;

extern crate clap;

use clap::{App, Arg};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

// Args for the program
// n the number of packets to generate
// lambda the average packet inter arrival time (in msec?) for poisson
// distribution
// mu the average packet processing time (in msec)

fn main() {
    let matches = App::new("Denarii")
        .version("0.1.0")
        .author("Taegyun Kim <k.taegyun@gmail.com>")
        .about("Packet based network device resource allocation simualtor.")
        .arg(
            Arg::with_name("seed")
                .short("s")
                .long("seed")
                .takes_value(true)
                .help("Random seed"),
        )
        .get_matches();

    let seed: u64 = matches
        .value_of("seed")
        .unwrap_or("1")
        .parse::<u64>()
        .unwrap();
    let mut rng: StdRng = SeedableRng::seed_from_u64(seed);

    let num_resources: u64 = 2;
    for x in 0..100 {
        // Generate random demand vectors
        let mut vec: Vec<i16> = Vec::new();
        for _y in 0..num_resources {
            vec.push(rng.gen_range(0, 1000));
        }
        println!("Hello, world {}, random vector: {:?}!", x, vec);
    }
}
