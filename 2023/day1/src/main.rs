fn main() {
    const INPUT_STR: &str = include_str!("../inputs/input.txt");

    let part_one_result = part_one(INPUT_STR);
    let part_two_result = part_two(INPUT_STR);

    println!("AOC Day 1");
    println!(" ---- ");
    println!("Part 1: {:?}", part_one_result);
    println!("Part 2: {:?}", part_two_result);
    println!(" ---- ");
}

fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split('\n')
            .filter(|v| !v.is_empty())
            .map(|l| {
                let mut first = 0;
                let mut last = 0;
                for c in l.chars() {
                    if let Some(v) = c.to_digit(10) {
                        first = v;
                        break;
                    }
                }
                for c in l.chars().rev() {
                    if let Some(v) = c.to_digit(10) {
                        last = v;
                        break;
                    }
                }
                first * 10 + last
            })
            .sum(),
    )
}

#[rustfmt::skip]
const WORD_TO_NUMBER: [(&'static str, u8); 9] = [
    ("one",    1),
    ("two",    2),
    ("three",  3),
    ("four",   4),
    ("five",   5),
    ("six",    6),
    ("seven",  7),
    ("eight",  8),
    ("nine",   9),
];

fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .split('\n')
            .filter(|v| !v.is_empty())
            .map(|l| {
                let mut first: u32 = 0;
                let mut last: u32 = 0;

                for i in 0..l.len() {
                    if let Some(c) = l.chars().nth(i) {
                        if let Some(v) = c.to_digit(10) {
                            first = v;
                            break;
                        }
                    }
                    let (sub_l, _) = l.split_at(i + 1);
                    for (label, val) in WORD_TO_NUMBER {
                        if let Some(_) = sub_l.find(label) {
                            first = val as u32;
                            break;
                        }
                    }

                    if first != 0 {
                        break;
                    }
                }

                for i in 0..l.len() {
                    if let Some(c) = l.chars().rev().nth(i) {
                        if let Some(v) = c.to_digit(10) {
                            last = v;
                            break;
                        }
                    }
                    let (_, sub_l) = l.split_at(l.len() - i - 1);
                    for (label, val) in WORD_TO_NUMBER {
                        if let Some(_) = sub_l.find(label) {
                            last = val as u32;
                            break;
                        }
                    }

                    if last != 0 {
                        break;
                    }
                }
                println!("{}: {} - {}", l, first, last);
                first * 10 + last
            })
            .sum(),
    )
}
