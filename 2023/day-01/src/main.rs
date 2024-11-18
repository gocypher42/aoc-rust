fn main() {
    const INPUT_STR: &str = include_str!("../inputs/input.txt");

    let part_one_result = part_one(INPUT_STR);
    let part_two_result = part_two(INPUT_STR);

    println!("AOC {}", env!("CARGO_PKG_NAME"));
    println!("-------");
    println!("Part 1: {part_one_result:?}");
    println!("Part 2: {part_two_result:?}");
    println!("-------");
}

fn part_one(input: &str) -> Option<u32> {
    let result = input
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
        .sum();
    Some(result)
}

const WORD_TO_NUMBER: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn get_val_from_subl(sub_l: &str) -> Option<u32> {
    for i in 0..WORD_TO_NUMBER.len() {
        if sub_l.contains(WORD_TO_NUMBER.get(i).unwrap()) {
            return Some((i as u32) + 1);
        }
    }
    None
}

fn part_two(input: &str) -> Option<u32> {
    let result = input
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
                get_val_from_subl(sub_l).map(|v| first = v);
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
                get_val_from_subl(sub_l).map(|v| last = v);
                if last != 0 {
                    break;
                }
            }
            first * 10 + last
        })
        .sum();
    Some(result)
}
