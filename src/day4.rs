use itertools::Itertools;
use regex::Regex;
use std::io::{self, BufRead};

use super::answer::Answer;

fn get_range<R: BufRead>(reader: &mut R) -> (u32, u32) {
    let pattern = Regex::new(r"^(?P<low>[0-9]+)-(?P<high>[0-9]+)$").unwrap();
    let mut buffer = String::new();
    let _num_bytes = reader.read_line(&mut buffer).unwrap();
    let captures = pattern.captures(&buffer.trim()).unwrap();
    (
        captures["low"].parse::<u32>().unwrap(),
        captures["high"].parse::<u32>().unwrap(),
    )
}

fn two_adjacent_identical(value: u32) -> bool {
    value
        .to_string()
        .chars()
        .tuple_windows()
        .any(|(ch1, ch2)| ch1 == ch2)
}

fn monotonic(value: u32) -> bool {
    value
        .to_string()
        .chars()
        .tuple_windows()
        .all(|(ch1, ch2)| ch1 <= ch2)
}

pub fn day4a<R: BufRead>(mut reader: &mut R) -> io::Result<Answer> {
    let (low, high) = get_range(&mut reader);
    Ok(Answer::US(
        (low..high)
            .filter(|&i| two_adjacent_identical(i))
            .filter(|&i| monotonic(i))
            .count(),
    ))
}

fn exactly_two_adjacent_identical(value: u32) -> bool {
    value
        .to_string()
        .chars()
        .group_by(|&ch| ch)
        .into_iter()
        .map(|(_key, group)| group.count())
        .any(|count| count == 2)
}

pub fn day4b<R: BufRead>(mut reader: &mut R) -> io::Result<Answer> {
    let (low, high) = get_range(&mut reader);
    Ok(Answer::US(
        (low..high)
            .filter(|&i| exactly_two_adjacent_identical(i))
            .filter(|&i| monotonic(i))
            .count(),
    ))
}
