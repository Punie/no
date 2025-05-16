mod reasons;

use clap::Parser;
use rand::prelude::*;

use reasons::REASONS;

#[derive(Parser)]
#[command(name = "no", version, about, long_about = None)]
struct Cli {}

fn random_reason() -> &'static str {
    let mut rng = rand::rng();

    REASONS.choose(&mut rng).unwrap()
}

fn main() {
    let _cli = Cli::parse();

    let reason = random_reason();

    println!("{reason}");
}
