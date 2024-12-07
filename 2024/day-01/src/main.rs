use std::collections::HashMap;

use utils::*;

fn main() {
    aoc_main!("../inputs/input.txt");
}

fn parse_pairs(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|l| {
            let (n1, n2) = l.split_once("   ").unwrap();
            let n1: usize = n1.parse().unwrap();
            let n2: usize = n2.parse().unwrap();
            (n1, n2)
        })
        .collect()
}

fn part_one(input: &str) -> usize {
    let pairs = parse_pairs(input);

    let mut l1: Vec<usize> = pairs.iter().map(|p| p.0).collect();
    l1.sort();

    let mut l2: Vec<usize> = pairs.iter().map(|p| p.1).collect();
    l2.sort();

    l1.iter().zip(l2.iter()).map(|(a, b)| a.abs_diff(*b)).sum()
}

fn part_two(input: &str) -> usize {
    let pairs = parse_pairs(input);

    let l1: Vec<usize> = pairs.iter().map(|p| p.0).collect();

    let mut l2_count = HashMap::new();
    for (_, n2) in &pairs {
        *l2_count.entry(*n2).or_insert(0) += 1;
    }

    l1.iter()
        .map(|&n1| n1 * *l2_count.get(&n1).unwrap_or(&0))
        .sum()
}
