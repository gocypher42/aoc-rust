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

    let max_steps = 25;

    let mut cache = HashMap::new();

    stones
        .iter()
        .map(|v| number_of_stone(*v, 0, max_steps, &mut cache))
        .sum()
}

fn part_two(input: &str) -> usize {
    let stones: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let max_steps = 75;

    let mut cache = HashMap::new();

    stones
        .iter()
        .map(|v| number_of_stone(*v, 0, max_steps, &mut cache))
        .sum()
}

fn number_of_stone(
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

    if stone == 0 {
        let s = 1;
        let v = number_of_stone(s, step + 1, max_step, cache);
        cache.entry((stone, step)).or_insert(v);
        return v;
    }

    let s = stone.to_string();
    if s.len() % 2 == 0 {
        let middle = s.len() / 2;
        let stone1 = s[..middle].parse().unwrap();
        let v1 = number_of_stone(stone1, step + 1, max_step, cache);

        let stone2 = s[middle..].parse().unwrap();
        let v2 = number_of_stone(stone2, step + 1, max_step, cache);

        let v = v1 + v2;
        cache.entry((stone, step)).or_insert(v);
        return v;
    }

    let v = number_of_stone(stone * 2024, step + 1, max_step, cache);
    cache.entry((stone, step)).or_insert(v);
    v
}
