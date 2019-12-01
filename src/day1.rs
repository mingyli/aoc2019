use std::cmp;
use std::io::{self, BufRead};

use super::answer::Answer;

cached! {
    FUEL_REQUIREMENT;
fn fuel_requirement(mass: i32) -> i32 = {
    cmp::max(mass / 3 - 2, 0)
}
}

pub fn day1a<R: BufRead>(reader: &mut R) -> io::Result<Answer> {
    Ok(Answer::I(
        reader
            .lines()
            .map(|line| line.unwrap())
            .map(|line| line.parse::<i32>().unwrap())
            .map(fuel_requirement)
            .sum(),
    ))
}

cached! {
    FUEL_REQUIREMENT_EXTENDED;
fn fuel_requirement_extended(mass: i32) -> i32 = {
    if mass == 0 {
        0
    } else {
        let fuel = fuel_requirement(mass);
        fuel + fuel_requirement_extended(fuel)
    }
}
}

pub fn day1b<R: BufRead>(reader: &mut R) -> io::Result<Answer> {
    Ok(Answer::I(
        reader
            .lines()
            .map(|line| line.unwrap())
            .map(|line| line.parse::<i32>().unwrap())
            .map(fuel_requirement_extended)
            .sum(),
    ))
}
