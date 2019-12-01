use std::env;

use aoc2019::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = config::Config::new(&args);

    let result = run::run(&config).unwrap();
    println!("result: {}", result);
}
