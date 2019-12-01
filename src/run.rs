use std::fs::File;
use std::io::{self, BufReader};

use crate::*;

pub fn run(config: &config::Config) -> io::Result<answer::Answer> {
    let file = File::open(config.filename.clone())?;
    let mut reader = BufReader::new(file);
    let solution = match config.problem.as_ref() {
        "day1a" => day1::day1a,
        "day1b" => day1::day1b,
        _ => day1::day1a,
    };
    solution(&mut reader)
}
