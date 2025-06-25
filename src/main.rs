use rand::prelude::*;
fn main() {
    let mut rng = rand::rng();
    println!("Random die roll: {}", rng.random_range(1..=10));
}
