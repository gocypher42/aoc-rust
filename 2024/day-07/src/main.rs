#[allow(clippy::wildcard_imports)]
use utils::*;

fn main() {
    aoc_main!("../inputs/input.txt");
}

fn part_one(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let (s1, s2) = l.split_once(':').unwrap();

            let target = s1.parse::<usize>().unwrap();
            let values = s2
                .trim()
                .split(' ')
                .map(|c| c.parse().unwrap())
                .collect::<Vec<usize>>();

            if possible_part1(values[0], &values[1..], target) {
                target
            } else {
                0
            }
        })
        .sum()
}

fn possible_part1(current: usize, values: &[usize], target: usize) -> bool {
    if values.is_empty() {
        return current == target;
    }

    if current > target {
        return false;
    }

    if possible_part1(current + values[0], &values[1..], target) {
        return true;
    }

    if possible_part1(current * values[0], &values[1..], target) {
        return true;
    }

    false
}

fn part_two(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let (s1, s2) = l.split_once(':').unwrap();

            let target = s1.parse::<usize>().unwrap();
            let values = s2
                .trim()
                .split(' ')
                .map(|c| c.parse().unwrap())
                .collect::<Vec<usize>>();

            if possible_part2(values[0], &values[1..], target) {
                target
            } else {
                0
            }
        })
        .sum()
}

fn possible_part2(current: usize, values: &[usize], target: usize) -> bool {
    if values.is_empty() {
        return current == target;
    }

    if current > target {
        return false;
    }

    if possible_part2(current + values[0], &values[1..], target) {
        return true;
    }

    if possible_part2(current * values[0], &values[1..], target) {
        return true;
    }

    if possible_part2(
        (current.to_string() + &values[0].to_string())
            .parse()
            .unwrap(),
        &values[1..],
        target,
    ) {
        return true;
    }

    false
}
