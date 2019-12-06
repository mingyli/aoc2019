use std::io::{self, BufRead};

use super::answer::Answer;

#[derive(Debug)]
enum Operation {
    Add,
    Mul,
    Input,
    Output,
    JumpTrue,
    JumpFalse,
    LessThan,
    Equals,
    End,
}

impl From<i32> for Operation {
    fn from(v: i32) -> Self {
        match v % 100 {
            1 => Operation::Add,
            2 => Operation::Mul,
            3 => Operation::Input,
            4 => Operation::Output,
            5 => Operation::JumpTrue,
            6 => Operation::JumpFalse,
            7 => Operation::LessThan,
            8 => Operation::Equals,
            _ => Operation::End,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Mode {
    Position,
    Immediate,
}

impl From<char> for Mode {
    fn from(ch: char) -> Self {
        match ch {
            '0' => Mode::Position,
            _ => Mode::Immediate,
        }
    }
}

fn get_modes(v: i32, op: Operation) -> Vec<Mode> {
    let bits: Vec<_> = match op {
        Operation::Add | Operation::Mul | Operation::LessThan | Operation::Equals => {
            format!("{:05}", v).chars().collect()
        }
        Operation::JumpTrue | Operation::JumpFalse => format!("{:04}", v).chars().collect(),
        Operation::Input | Operation::Output => format!("{:03}", v).chars().collect(),
        _ => "".chars().collect(),
    };
    bits.iter().rev().skip(2).cloned().map(Mode::from).collect()
}

fn resolve_mode(tokens: &[i32], mode: Mode, val: i32) -> i32 {
    match mode {
        Mode::Immediate => val,
        Mode::Position => tokens[val as usize],
    }
}

fn compute(tokens: &mut [i32], input: i32) -> Vec<i32> {
    let mut results: Vec<i32> = Vec::new();
    let mut pos: usize = 0;
    while pos < tokens.len() {
        let op = Operation::from(tokens[pos]);
        let modes = get_modes(tokens[pos], Operation::Add);
        match op {
            Operation::Add => {
                let i1 = resolve_mode(tokens, modes[0], tokens[pos + 1]);
                let i2 = resolve_mode(tokens, modes[1], tokens[pos + 2]);
                let i3 = tokens[pos + 3] as usize;
                tokens[i3] = i1 + i2;
                pos += 4;
            }
            Operation::Mul => {
                let i1 = resolve_mode(tokens, modes[0], tokens[pos + 1]);
                let i2 = resolve_mode(tokens, modes[1], tokens[pos + 2]);
                let i3 = tokens[pos + 3] as usize;
                tokens[i3] = i1 * i2;
                pos += 4;
            }
            Operation::Input => {
                let dest = tokens[pos + 1] as usize;
                tokens[dest] = input;
                pos += 2;
            }
            Operation::Output => {
                let src = tokens[pos + 1] as usize;
                results.push(tokens[src]);
                pos += 2;
            }
            Operation::JumpTrue => {
                let i1 = resolve_mode(tokens, modes[0], tokens[pos + 1]);
                let i2 = resolve_mode(tokens, modes[1], tokens[pos + 2]);
                pos = if i1 != 0 { i2 as usize } else { pos + 3 };
            }
            Operation::JumpFalse => {
                let i1 = resolve_mode(tokens, modes[0], tokens[pos + 1]);
                let i2 = resolve_mode(tokens, modes[1], tokens[pos + 2]);
                pos = if i1 == 0 { i2 as usize } else { pos + 3 };
            }
            Operation::LessThan => {
                let i1 = resolve_mode(tokens, modes[0], tokens[pos + 1]);
                let i2 = resolve_mode(tokens, modes[1], tokens[pos + 2]);
                let i3 = tokens[pos + 3] as usize;
                tokens[i3] = if i1 < i2 { 1 } else { 0 };
                pos += 4;
            }
            Operation::Equals => {
                let i1 = resolve_mode(tokens, modes[0], tokens[pos + 1]);
                let i2 = resolve_mode(tokens, modes[1], tokens[pos + 2]);
                let i3 = tokens[pos + 3] as usize;
                tokens[i3] = if i1 == i2 { 1 } else { 0 };
                pos += 4;
            }
            Operation::End => {
                break;
            }
        }
    }
    results
}

fn get_tokens<R: BufRead>(reader: &mut R) -> Vec<i32> {
    let mut buffer = String::new();
    let _num_bytes = reader.read_line(&mut buffer).unwrap();
    let tokens: Vec<i32> = buffer
        .trim()
        .split(',')
        .map(|t| t.parse::<i32>().unwrap())
        .collect();
    tokens
}

pub fn day5a<R: BufRead>(mut reader: &mut R) -> io::Result<Answer> {
    let mut tokens = get_tokens(&mut reader);
    let output = compute(&mut tokens, 1);
    Ok(Answer::I(output.last().cloned().unwrap()))
}

pub fn day5b<R: BufRead>(mut reader: &mut R) -> io::Result<Answer> {
    let mut tokens = get_tokens(&mut reader);
    let output = compute(&mut tokens, 5);
    Ok(Answer::I(output.last().cloned().unwrap()))
}
