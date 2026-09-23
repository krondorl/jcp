// Copyright 2024-present Adam Burucs. MIT license.

mod adjectives;
mod generator;
mod nouns;
mod verbs;

use generator::generate_sentence;
use tinyrand::{Rand, Seeded, StdRand};
use tinyrand_std::clock_seed::ClockSeed;
use verbs::VerbType;

fn main() {
    println!();
    println!("John Carmack Planner");
    println!("Generate random plan files in John Carmack style.");
    println!();
    println!("Generating plan notes...");
    println!();
    let seed = ClockSeed.next_u64();
    let mut rng = StdRand::seed(seed);
    for _i in 0..14 {
        let sentence = generate_sentence(VerbType::Past, &mut rng);
        println!("{sentence}");
    }
}
