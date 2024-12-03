use utils::*;

fn main() {
    aoc_main!("../inputs/input.txt");
}

fn part_one(input: &str) -> usize {
    let current_slice = input.trim();

    current_slice
        .chars()
        .enumerate()
        .filter_map(|(i, c)| match c {
            'm' => parse_mul(&current_slice[i..]),
            _ => None,
        })
        .sum()
}

fn part_two(input: &str) -> usize {
    let mut enable = true;

    let current_slice = input.trim();

    current_slice
        .chars()
        .enumerate()
        .filter_map(|(i, c)| {
            let sub = &current_slice[i..];
            match c {
                'd' => {
                    if enable && sub.find("don't()") == Some(0) {
                        enable = false;
                    }

                    if !enable && sub.find("do()") == Some(0) {
                        enable = true;
                    }
                    None
                }
                'm' => {
                    if !enable {
                        None
                    } else {
                        parse_mul(sub)
                    }
                }
                _ => None,
            }
        })
        .sum()
}

const MUL_PAREN: &str = "mul(";
const PAREN: &str = ")";
const COMMA: u8 = b',';

fn parse_mul(sub: &str) -> Option<usize> {
    if sub.find(MUL_PAREN) != Some(0) {
        return None;
    }

    let end_pos = match sub.find(PAREN) {
        Some(val) => val,
        None => return None,
    };

    if let Some(_) = sub[MUL_PAREN.len()..end_pos].find(MUL_PAREN) {
        return None;
    }

    let comma_pos = match sub.as_bytes().iter().position(|b| *b == COMMA) {
        Some(v) => v,
        None => return None,
    };

    let num1: usize = match sub[MUL_PAREN.len()..comma_pos].parse() {
        Ok(val) => val,
        Err(_) => {
            return None;
        }
    };

    let num2: usize = match sub[comma_pos + 1..end_pos].parse() {
        Ok(val) => val,
        Err(_) => {
            return None;
        }
    };

    return Some(num1 * num2);
}
