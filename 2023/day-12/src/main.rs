use std::fmt::Debug;

use utils::*;

fn main() {
    aoc_main!("../inputs/input.txt");
}

#[derive(PartialEq, Eq, Clone)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl Debug for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Spring::Operational => ".",
                Spring::Damaged => "#",
                Spring::Unknown => "?",
            }
        )
    }
}

struct Record {
    springs: Vec<Spring>,
    expected_dammaged: Vec<u32>,
}

impl Debug for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.springs, self.expected_dammaged)
    }
}

fn part_one(input: &str) -> usize {
    let records = parse(input);

    let mut sum = 0;
    for x in records {
        let count = count_possible_arangements(x.springs.clone(), x.expected_dammaged.clone());
        // println!("{x:?} {count}");
        sum += count;
    }
    sum as usize
}

fn part_two(input: &str) -> usize {
    let mut records = parse(input);
    records.iter_mut().for_each(|r| {
        // println!("{r:?}");
        let s = std::mem::take(&mut r.springs);
        let e = std::mem::take(&mut r.expected_dammaged);

        for _ in 0..5 {
            r.springs.append(&mut s.clone());
            r.springs.push(Spring::Unknown);
            r.expected_dammaged.append(&mut e.clone());
        }

        r.springs.pop();
        // println!("{r:?}");
    });

    records
        .iter()
        .enumerate()
        .map(|(i, x)| {
            let count = count_possible_arangements(x.springs.clone(), x.expected_dammaged.clone());
            // println!("{x:?} {count} {i}");
            count
        })
        .sum::<usize>()
}

fn parse(input: &str) -> Vec<Record> {
    input
        .lines()
        .map(|l| {
            let (springs, expected_dammaged) = l.split_once(' ').unwrap();

            Record {
                springs: springs
                    .chars()
                    .map(|c| match c {
                        '#' => Spring::Damaged,
                        '?' => Spring::Unknown,
                        '.' => Spring::Operational,
                        _ => unreachable!(),
                    })
                    .collect(),
                expected_dammaged: expected_dammaged
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect(),
            }
        })
        .collect()
}

fn count_possible_arangements(mut springs: Vec<Spring>, counts: Vec<u32>) -> usize {
    springs.push(Spring::Operational);
    let mut cache = vec![vec![None; springs.len()]; counts.len()];
    count_possible_arangements_inner(&springs, &counts, &mut cache)
}

fn count_possible_arangements_inner(
    springs: &[Spring],
    counts: &[u32],
    cache: &mut [Vec<Option<usize>>],
) -> usize {
    if counts.is_empty() {
        return if springs.contains(&Spring::Damaged) {
            0
        } else {
            1
        };
    }

    if springs.len() < (counts.iter().sum::<u32>() as usize) + counts.len() {
        return 0;
    }

    if let Some(cached) = cache[counts.len() - 1][springs.len() - 1] {
        return cached;
    }

    let mut arangements = 0;
    if springs[0] != Spring::Damaged {
        arangements += count_possible_arangements_inner(&springs[1..], counts, cache);
    }

    let next_group_size = counts[0] as usize;
    if !springs[..next_group_size].contains(&Spring::Operational)
        && springs[next_group_size] != Spring::Damaged
    {
        arangements +=
            count_possible_arangements_inner(&springs[next_group_size + 1..], &counts[1..], cache);
    }
    cache[counts.len() - 1][springs.len() - 1] = Some(arangements);
    arangements
}
