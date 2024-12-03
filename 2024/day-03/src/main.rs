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

fn parse_mul(sub: &str) -> Option<usize> {
    if sub.find(MUL_PAREN) != Some(0) {
        return None;
    }

    let end_pos = match sub.find(")") {
        Some(v) => v,
        None => return None,
    };

    if sub[MUL_PAREN.len()..end_pos].find(MUL_PAREN).is_some() {
        return None;
    }

    let comma_pos = match sub.chars().position(|b| b == ',') {
        Some(v) => v,
        None => return None,
    };

    let num1: usize = match sub[MUL_PAREN.len()..comma_pos].parse() {
        Ok(v) => v,
        Err(_) => {
            return None;
        }
    };

    let num2: usize = match sub[comma_pos + 1..end_pos].parse() {
        Ok(v) => v,
        Err(_) => {
            return None;
        }
    };

    return Some(num1 * num2);
}
