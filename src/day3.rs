use im::hashmap::HashMap;
use regex::Regex;
use std::collections::HashSet;
use std::convert::TryInto;
use std::io::{self, BufRead};
use std::iter;

use super::answer::Answer;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from(ch: char) -> Option<Direction> {
        match ch {
            'U' => Some(Direction::Up),
            'D' => Some(Direction::Down),
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            _ => None,
        }
    }

    fn advance(self, (row, col): Point) -> Point {
        match self {
            Direction::Up => (row - 1, col),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
            Direction::Right => (row, col + 1),
        }
    }
}

#[derive(Debug)]
struct Step {
    direction: Direction,
    amount: i32,
}

impl Step {
    fn from(string: &str) -> Step {
        let pattern = Regex::new(r"^(?P<direction>.)(?P<amount>[0-9]+)$").unwrap();
        let captures = pattern.captures(string).unwrap();
        Step {
            direction: Direction::from(captures["direction"].chars().next().unwrap()).unwrap(),
            amount: captures["amount"].parse::<i32>().unwrap(),
        }
    }
}

fn get_steps<R: BufRead>(reader: &mut R) -> Vec<Step> {
    let mut buffer = String::new();
    let _num_bytes = reader.read_line(&mut buffer).unwrap();
    let tokens: Vec<Step> = buffer
        .trim()
        .split(',')
        .map(|t| Step::from(&t.to_string()))
        .collect();
    tokens
}

type Point = (i32, i32);

fn get_path(steps: &[Step]) -> HashSet<Point> {
    steps
        .iter()
        .flat_map(|step| iter::repeat(step.direction).take(step.amount.try_into().unwrap()))
        .scan((0, 0), |curr, direction| {
            *curr = direction.advance(*curr);
            Some(*curr)
        })
        .collect()
}

pub fn day3a<R: BufRead>(mut reader: &mut R) -> io::Result<Answer> {
    let steps1 = get_steps(&mut reader);
    let steps2 = get_steps(&mut reader);
    let path1 = get_path(&steps1);
    let path2 = get_path(&steps2);
    let intersection: HashSet<_> = path1.intersection(&path2).collect();
    Ok(Answer::I(
        intersection
            .iter()
            .map(|(r, c)| r.abs() + c.abs())
            .min()
            .unwrap(),
    ))
}

fn get_enumerated_path(steps: &[Step]) -> HashMap<Point, usize> {
    steps
        .iter()
        .flat_map(|step| iter::repeat(step.direction).take(step.amount.try_into().unwrap()))
        .scan((0, 0), |curr, direction| {
            *curr = direction.advance(*curr);
            Some(*curr)
        })
        .enumerate()
        .map(|(i, p)| (p, i + 1))
        .collect()
}

pub fn day3b<R: BufRead>(mut reader: &mut R) -> io::Result<Answer> {
    let steps1 = get_steps(&mut reader);
    let steps2 = get_steps(&mut reader);
    let path1 = get_enumerated_path(&steps1);
    let path2 = get_enumerated_path(&steps2);
    let intersection: HashMap<_, _> = path1.intersection_with(path2, |v1, v2| v1 + v2);
    Ok(Answer::US(*intersection.values().min().unwrap()))
}
