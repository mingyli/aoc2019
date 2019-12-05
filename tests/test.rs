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

#[test]
fn test_day2a() {
    let config = config::Config {
        problem: "day2a".to_string(),
        filename: "./input/day2".to_string(),
    };
    assert_eq!(4330636, run::run(&config).unwrap());
}

#[test]
fn test_day2b() {
    let config = config::Config {
        problem: "day2b".to_string(),
        filename: "./input/day2".to_string(),
    };
    assert_eq!(6086, run::run(&config).unwrap());
}

#[test]
fn test_day3a() {
    let config = config::Config {
        problem: "day3a".to_string(),
        filename: "./input/day3".to_string(),
    };
    assert_eq!(316, run::run(&config).unwrap());
}

#[test]
fn test_day3b() {
    let config = config::Config {
        problem: "day3b".to_string(),
        filename: "./input/day3".to_string(),
    };
    assert_eq!(16368, run::run(&config).unwrap());
}
