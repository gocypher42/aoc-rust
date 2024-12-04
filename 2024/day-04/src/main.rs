#[allow(clippy::wildcard_imports)]
use utils::*;

fn main() {
    aoc_main!("../inputs/input.txt");
}

fn is_xmas(word: &[&char]) -> bool {
    word == &[&'X', &'M', &'A', &'S']
}

fn part_one(input: &str) -> usize {
    let data: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut count = 0;

    for (j, row) in data.iter().enumerate() {
        for (i, c) in row.iter().enumerate() {
            if *c != 'X' {
                continue;
            }

            // Check diag down right
            let x = data[j..]
                .iter()
                .take(4)
                .enumerate()
                .filter_map(|(k, l)| l.get(i + k))
                .collect::<Vec<&char>>();
            if is_xmas(x.as_slice()) {
                count += 1;
            }

            // Check vertical forward
            let x = row[i..].iter().take(4).collect::<Vec<&char>>();
            if is_xmas(x.as_slice()) {
                count += 1;
            }

            // Check diag down left
            let x = data[j..]
                .iter()
                .take(4)
                .enumerate()
                .filter_map(|(k, l)| if i >= 3 { l.get(i - k) } else { None })
                .collect::<Vec<&char>>();
            if is_xmas(x.as_slice()) {
                count += 1;
            }

            // Check vertical backward
            if i >= 3 {
                let x = row.iter().skip(i - 3).take(4).rev().collect::<Vec<&char>>();
                if is_xmas(x.as_slice()) {
                    count += 1;
                }
            }

            // Check horizontal downward
            let x = data[j..]
                .iter()
                .take(4)
                .filter_map(|l| l.get(i))
                .collect::<Vec<&char>>();
            if is_xmas(x.as_slice()) {
                count += 1;
            }

            if j >= 3 {
                // Check horizontal upward
                let x = data
                    .iter()
                    .skip(j - 3)
                    .take(4)
                    .filter_map(|l| l.get(i))
                    .rev()
                    .collect::<Vec<&char>>();
                if is_xmas(x.as_slice()) {
                    count += 1;
                }

                // Check diag up right
                let x = data
                    .iter()
                    .skip(j - 3)
                    .take(4)
                    .rev()
                    .enumerate()
                    .filter_map(|(k, l)| l.get(i + k))
                    .collect::<Vec<&char>>();
                if is_xmas(x.as_slice()) {
                    count += 1;
                }
            }

            // Check diag up left
            if j >= 3 && i >= 3 {
                let x = data
                    .iter()
                    .skip(j - 3)
                    .take(4)
                    .rev()
                    .enumerate()
                    .filter_map(|(k, l)| l.get(i - k))
                    .collect::<Vec<&char>>();
                if is_xmas(x.as_slice()) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn part_two(input: &str) -> usize {
    let data: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    data.iter()
        .enumerate()
        .take(data.len() - 1)
        .skip(1)
        .flat_map(|(j, row)| {
            let data_ref = &data;
            row.iter()
                .enumerate()
                .take(row.len() - 1)
                .skip(1)
                .filter_map(move |(i, &c)| {
                    if c != 'A' {
                        return None;
                    }

                    let bs = [
                        data_ref.get(j - 1)?.get(i - 1)?,
                        data_ref.get(j + 1)?.get(i + 1)?,
                    ];
                    let fs = [
                        data_ref.get(j - 1)?.get(i + 1)?,
                        data_ref.get(j + 1)?.get(i - 1)?,
                    ];

                    if ((bs == [&'M', &'S']) || (bs == [&'S', &'M']))
                        && ((fs == [&'M', &'S']) || (fs == [&'S', &'M']))
                    {
                        return Some(());
                    }

                    None
                })
        })
        .count()
}
