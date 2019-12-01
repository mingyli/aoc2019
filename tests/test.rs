extern crate aoc2019;
use aoc2019::{config, run};

#[test]
fn test_day1a() {
    let config = config::Config {
        problem: "day1a".to_string(),
        filename: "./input/day1".to_string(),
    };
    assert_eq!(3287620, run::run(&config).unwrap());
}

#[test]
fn test_day1b() {
    let config = config::Config {
        problem: "day1b".to_string(),
        filename: "./input/day1".to_string(),
    };
    assert_eq!(4928567, run::run(&config).unwrap());
}
