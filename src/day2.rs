use itertools::Itertools;
use std::io::{self, BufRead, Error, ErrorKind};

use super::answer::Answer;

const ADD_OP: usize = 1;
const MUL_OP: usize = 2;
const END_OP: usize = 99;

fn compute(tokens: &mut [usize]) -> io::Result<usize> {
    for pos in (0..tokens.len()).step_by(4) {
        match tokens[pos] {
            ADD_OP => {
                let (i1, i2, i3) = (tokens[pos + 1], tokens[pos + 2], tokens[pos + 3]);
                tokens[i3] = tokens[i1] + tokens[i2];
            }
            MUL_OP => {
                let (i1, i2, i3) = (tokens[pos + 1], tokens[pos + 2], tokens[pos + 3]);
                tokens[i3] = tokens[i1] * tokens[i2];
            }
            END_OP => {
                break;
            }
            _ => {
                return Err(Error::new(ErrorKind::InvalidInput, "Bad program."));
            }
        }
    }
    Ok(tokens[0])
}

fn get_tokens<R: BufRead>(reader: &mut R) -> Vec<usize> {
    let mut buffer = String::new();
    let _num_bytes = reader.read_line(&mut buffer).unwrap();
    let tokens: Vec<usize> = buffer
        .trim()
        .split(',')
        .map(|t| t.parse::<usize>().unwrap())
        .collect();
    tokens
}

pub fn day2a<R: BufRead>(mut reader: &mut R) -> io::Result<Answer> {
    let mut tokens = get_tokens(&mut reader);
    tokens[1] = 12;
    tokens[2] = 2;
    Ok(Answer::US(compute(&mut tokens)?))
}

pub fn day2b<R: BufRead>(mut reader: &mut R) -> io::Result<Answer> {
    const DESIRED_OUTPUT: usize = 19_690_720;
    let tokens = get_tokens(&mut reader);
    let (noun, verb) = (0..100)
        .cartesian_product(0..100)
        .find(|&(noun, verb)| {
            let mut new_tokens = tokens.clone();
            new_tokens[1] = noun;
            new_tokens[2] = verb;
            compute(&mut new_tokens).unwrap() == DESIRED_OUTPUT
        })
        .unwrap();
    Ok(Answer::US(100 * noun + verb))
}
