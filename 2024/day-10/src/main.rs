#[allow(clippy::wildcard_imports)]
use utils::*;

fn main() {
    aoc_main!("../inputs/input.txt");
}

fn part_one(input: &str) -> usize {
    let grid: Vec<Vec<Option<u8>>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).map(|v| u8::try_from(v).unwrap()))
                .collect()
        })
        .collect();

    let starts: Vec<Position> = grid
        .iter()
        .enumerate()
        .flat_map(|(j, row)| {
            row.iter().enumerate().filter_map(move |(i, cell)| {
                if let Some(0) = cell {
                    Some(Position { x: i, y: j })
                } else {
                    None
                }
            })
        })
        .collect();

    let mut trailheads: Vec<(Position, Position)> = vec![];

    for start in starts {
        let mut possible_next_pos: Vec<Position> = vec![start.clone()];

        while let Some(pos) = possible_next_pos.pop() {
            let current_value = grid[pos.y][pos.x].unwrap();
            if current_value == 9 {
                let pair = (start.clone(), pos);
                if !trailheads.contains(&pair) {
                    trailheads.push(pair);
                }
                continue;
            }

            if pos.x > 0 && grid[pos.y][pos.x - 1].unwrap_or(0) == current_value + 1 {
                let mut new_pos = pos.clone();
                new_pos.x -= 1;
                possible_next_pos.push(new_pos);
            }
            if pos.x < grid[0].len() - 1 && grid[pos.y][pos.x + 1].unwrap_or(0) == current_value + 1
            {
                let mut new_pos = pos.clone();
                new_pos.x += 1;
                possible_next_pos.push(new_pos);
            }
            if pos.y > 0 && grid[pos.y - 1][pos.x].unwrap_or(0) == current_value + 1 {
                let mut new_pos = pos.clone();
                new_pos.y -= 1;
                possible_next_pos.push(new_pos);
            }
            if pos.y < grid.len() - 1 && grid[pos.y + 1][pos.x].unwrap_or(0) == current_value + 1 {
                let mut new_pos = pos.clone();
                new_pos.y += 1;
                possible_next_pos.push(new_pos);
            }
        }
    }

    trailheads.len()
}

fn part_two(input: &str) -> usize {
    let grid: Vec<Vec<Option<u8>>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).map(|v| u8::try_from(v).unwrap()))
                .collect()
        })
        .collect();

    let starts: Vec<Position> = grid
        .iter()
        .enumerate()
        .flat_map(|(j, row)| {
            row.iter().enumerate().filter_map(move |(i, cell)| {
                if let Some(0) = cell {
                    Some(Position { x: i, y: j })
                } else {
                    None
                }
            })
        })
        .collect();

    let mut count = 0;

    for start in starts {
        let mut possible_next_pos: Vec<Position> = vec![start.clone()];

        while let Some(pos) = possible_next_pos.pop() {
            let current_value = grid[pos.y][pos.x].unwrap();
            if current_value == 9 {
                count += 1;
                continue;
            }

            if pos.x > 0 && grid[pos.y][pos.x - 1].unwrap_or(0) == current_value + 1 {
                let mut new_pos = pos.clone();
                new_pos.x -= 1;
                possible_next_pos.push(new_pos);
            }
            if pos.x < grid[0].len() - 1 && grid[pos.y][pos.x + 1].unwrap_or(0) == current_value + 1
            {
                let mut new_pos = pos.clone();
                new_pos.x += 1;
                possible_next_pos.push(new_pos);
            }
            if pos.y > 0 && grid[pos.y - 1][pos.x].unwrap_or(0) == current_value + 1 {
                let mut new_pos = pos.clone();
                new_pos.y -= 1;
                possible_next_pos.push(new_pos);
            }
            if pos.y < grid.len() - 1 && grid[pos.y + 1][pos.x].unwrap_or(0) == current_value + 1 {
                let mut new_pos = pos.clone();
                new_pos.y += 1;
                possible_next_pos.push(new_pos);
            }
        }
    }

    count
}
