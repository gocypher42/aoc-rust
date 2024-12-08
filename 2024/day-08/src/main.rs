use std::{collections::HashMap, vec};

#[allow(clippy::wildcard_imports)]
use utils::*;

fn main() {
    aoc_main!("../inputs/input.txt");
}

#[derive(PartialEq, Eq, Clone)]
enum Tile {
    Freq(char),
    Empty,
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Freq(c) => write!(f, "{c}"),
            Tile::Empty => write!(f, "."),
        }
    }
}

fn part_one(input: &str) -> usize {
    let grid: Vec<Vec<Tile>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    v => Tile::Freq(v),
                })
                .collect()
        })
        .collect();

    let mut grid_anti: Vec<Vec<Tile>> = vec![vec![Tile::Empty; grid[0].len()]; grid.len()];

    let mut map: HashMap<char, Vec<Position>> = HashMap::new();
    grid.iter().enumerate().for_each(|(j, row)| {
        row.iter().enumerate().for_each(|(i, cell)| match cell {
            Tile::Freq(val) => map.entry(*val).or_default().push(Position { x: i, y: j }),
            Tile::Empty => {}
        });
    });

    for (_, positions) in map {
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let p1 = &positions[i];
                let p2 = &positions[j];

                let p1_dx: isize = (p1.x as isize) - (p2.x as isize);
                let p1_dy: isize = (p1.y as isize) - (p2.y as isize);
                if let Some(row) = grid_anti.get_mut((p1.y as isize + p1_dy) as usize) {
                    if let Some(cell) = row.get_mut((p1.x as isize + p1_dx) as usize) {
                        if *cell == Tile::Empty {
                            *cell = Tile::Freq('#');
                        }
                    }
                }

                let p2_dx: isize = (p2.x as isize) - (p1.x as isize);
                let p2_dy: isize = (p2.y as isize) - (p1.y as isize);
                if let Some(row) = grid_anti.get_mut((p2.y as isize + p2_dy) as usize) {
                    if let Some(cell) = row.get_mut((p2.x as isize + p2_dx) as usize) {
                        if *cell == Tile::Empty {
                            *cell = Tile::Freq('#');
                        }
                    }
                }
            }
        }
    }

    grid_anti
        .iter()
        .flat_map(|row| {
            row.iter().filter_map(|cell| {
                if *cell == Tile::Freq('#') {
                    Some(())
                } else {
                    None
                }
            })
        })
        .count()
}

fn part_two(input: &str) -> usize {
    let grid: Vec<Vec<Tile>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    v => Tile::Freq(v),
                })
                .collect()
        })
        .collect();

    let mut grid_anti: Vec<Vec<Tile>> = vec![vec![Tile::Empty; grid[0].len()]; grid.len()];

    let mut map: HashMap<char, Vec<Position>> = HashMap::new();
    grid.iter().enumerate().for_each(|(j, row)| {
        row.iter().enumerate().for_each(|(i, cell)| match cell {
            Tile::Freq(val) => {
                map.entry(*val).or_default().push(Position { x: i, y: j });
                grid_anti[j][i] = Tile::Freq('#');
            }
            Tile::Empty => {}
        });
    });

    for (_, positions) in map {
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let p1 = &positions[i];
                let p2 = &positions[j];

                let p1_dx: isize = (p1.x as isize) - (p2.x as isize);
                let p1_dy: isize = (p1.y as isize) - (p2.y as isize);

                let mut c = 1;
                while let Some(row) = grid_anti.get_mut((p1.y as isize + p1_dy * c) as usize) {
                    if let Some(cell) = row.get_mut((p1.x as isize + p1_dx * c) as usize) {
                        if *cell == Tile::Empty {
                            *cell = Tile::Freq('#');
                        }
                    }
                    c += 1;
                }

                let p2_dx: isize = (p2.x as isize) - (p1.x as isize);
                let p2_dy: isize = (p2.y as isize) - (p1.y as isize);

                let mut c = 1;
                while let Some(row) = grid_anti.get_mut((p2.y as isize + p2_dy * c) as usize) {
                    if let Some(cell) = row.get_mut((p2.x as isize + p2_dx * c) as usize) {
                        if *cell == Tile::Empty {
                            *cell = Tile::Freq('#');
                        }
                    }
                    c += 1;
                }
            }
        }
    }

    grid_anti
        .iter()
        .flat_map(|row| {
            row.iter().filter_map(|cell| {
                if *cell == Tile::Freq('#') {
                    Some(())
                } else {
                    None
                }
            })
        })
        .count()
}
