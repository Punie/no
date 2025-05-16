use rand::prelude::*;

mod reasons;

fn main() {
    let mut rng = rand::rng();

    let reason = reasons::REASONS.choose(&mut rng).unwrap();

    println!("{reason}");
}
