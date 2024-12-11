use std::collections::HashMap;

#[allow(clippy::wildcard_imports)]
use utils::*;

fn main() {
    aoc_main!("../inputs/input.txt");
}

fn part_one(input: &str) -> usize {
    let stones: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut cache = HashMap::new();

    stones
        .iter()
        .map(|v| number_of_stones(*v, 0, 25, &mut cache))
        .sum()
}

fn part_two(input: &str) -> usize {
    let stones: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut cache = HashMap::new();

    stones
        .iter()
        .map(|v| number_of_stones(*v, 0, 75, &mut cache))
        .sum()
}

fn number_of_stones(
    stone: usize,
    step: usize,
    max_step: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if step == max_step {
        return 1;
    }

    if let Some(v) = cache.get(&(stone, step)) {
        return *v;
    }

    let v = if stone == 0 {
        number_of_stones(1, step + 1, max_step, cache)
    } else {
        let s = stone.to_string();
        if s.len() % 2 == 0 {
            let middle = s.len() / 2;
            number_of_stones(s[..middle].parse().unwrap(), step + 1, max_step, cache)
                + number_of_stones(s[middle..].parse().unwrap(), step + 1, max_step, cache)
        } else {
            number_of_stones(stone * 2024, step + 1, max_step, cache)
        }
    };

    cache.entry((stone, step)).or_insert(v);
    v
}
