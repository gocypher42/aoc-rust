#[allow(clippy::wildcard_imports)]
use utils::*;

fn main() {
    aoc_main!("../inputs/input.txt");
}

fn is_xmas(word: &[&char]) -> bool {
    word == [&'X', &'M', &'A', &'S']
}

fn part_one(input: &str) -> usize {
    let mut data_t: Vec<Vec<char>> = input
        .lines()
        .map(|l| {
            let mut cline = vec!['.'; 4];
            cline.append(&mut l.chars().collect());
            cline.append(&mut vec!['.'; 4]);
            cline
        })
        .collect();
    let mut data = vec![vec!['.'; data_t[0].len()]; 4];
    data.append(&mut data_t);
    data.append(&mut vec![vec!['.'; data[0].len()]; 4]);

    let data = data;
    let mut count = 0;

    for (j, row) in data.iter().enumerate() {
        for (i, c) in row.iter().enumerate() {
            if *c != 'X' {
                continue;
            }

            // Check diag down right
            let x: Vec<&char> = data[j..]
                .iter()
                .take(4)
                .enumerate()
                .filter_map(|(k, l)| l.get(i + k))
                .collect();
            if is_xmas(x.as_slice()) {
                count += 1;
            }

            // Check vertical forward
            let x: Vec<&char> = row[i..].iter().take(4).collect();
            if is_xmas(x.as_slice()) {
                count += 1;
            }

            // Check diag down left
            let x: Vec<&char> = data[j..]
                .iter()
                .take(4)
                .enumerate()
                .filter_map(|(k, l)| l.get(i - k))
                .collect();
            if is_xmas(x.as_slice()) {
                count += 1;
            }

            // Check vertical backward
            let x: Vec<&char> = row.iter().skip(i - 3).take(4).rev().collect();
            if is_xmas(x.as_slice()) {
                count += 1;
            }

            // Check horizontal downward
            let x: Vec<&char> = data[j..].iter().take(4).filter_map(|l| l.get(i)).collect();
            if is_xmas(x.as_slice()) {
                count += 1;
            }

            // Check horizontal upward
            let x: Vec<&char> = data
                .iter()
                .skip(j - 3)
                .take(4)
                .filter_map(|l| l.get(i))
                .rev()
                .collect();
            if is_xmas(x.as_slice()) {
                count += 1;
            }

            // Check diag up right
            let x: Vec<&char> = data
                .iter()
                .skip(j - 3)
                .take(4)
                .rev()
                .enumerate()
                .filter_map(|(k, l)| l.get(i + k))
                .collect();
            if is_xmas(x.as_slice()) {
                count += 1;
            }

            // Check diag up left
            let x: Vec<&char> = data
                .iter()
                .skip(j - 3)
                .take(4)
                .rev()
                .enumerate()
                .filter_map(|(k, l)| l.get(i - k))
                .collect();
            if is_xmas(x.as_slice()) {
                count += 1;
            }
        }
    }

    count
}

fn part_two(input: &str) -> usize {
    let mut data_t: Vec<Vec<char>> = input
        .lines()
        .map(|l| {
            let mut cline = vec!['.'; 1];
            cline.append(&mut l.chars().collect());
            cline.push('.');
            cline
        })
        .collect();
    let mut data = vec![vec!['.'; data_t[0].len()]; 1];
    data.append(&mut data_t);
    data.push(vec!['.'; data[0].len()]);

    let data = data;

    data.iter()
        .enumerate()
        .flat_map(|(j, row)| {
            let data_ref = &data;
            row.iter().enumerate().filter_map(move |(i, &c)| {
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
