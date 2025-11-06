use rand::Rng;
use std::io::{stdout, Write};

fn main() {
    let mut rng = rand::rng();
    let seed: i128 = rng.random_range(i128::MIN..=i128::MAX);

    print!("Generated seed: {}", seed);
    stdout().flush().unwrap(); // Force output to appear
}