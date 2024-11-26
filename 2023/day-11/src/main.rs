use std::collections::HashMap;
use std::fmt::Debug;

use utils::*;

fn main() {
   aoc_main!("../inputs/input.txt");
}

#[derive(PartialEq, Eq, Clone)]
enum Tile {
    Space,
    Galaxy(u32),
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Space => '.'.to_string(),
                Tile::Galaxy(v) => v.to_string(),
            }
        )
    }
}

fn part_one(input: &str) -> usize {
    let mut count = 0;
    let mut galaxie: Vec<Vec<Tile>> = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Space,
                    '#' => {
                        count += 1;
                        Tile::Galaxy(count)
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    // print_2d_slice(&galaxie);

    let lines_to_expand: Vec<usize> = galaxie
        .iter()
        .enumerate()
        .filter_map(|(i, line)| {
            if line.iter().all(|t| *t == Tile::Space) {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    let cols_to_expand: Vec<usize> = galaxie[0]
        .iter()
        .enumerate()
        .filter_map(|(i, c)| {
            if *c == Tile::Space && galaxie.iter().all(|line| line[i] == Tile::Space) {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    // println!("{:?}", lines_to_expand);
    // println!("{:?}", cols_to_expand);

    for j in lines_to_expand.iter().rev() {
        galaxie.insert(*j, vec![Tile::Space; galaxie[0].len()]);
    }

    for i in cols_to_expand.iter().rev() {
        galaxie
            .iter_mut()
            .for_each(|line| line.insert(*i, Tile::Space));
    }

    // print_2d_slice(&galaxie);

    let galaxie = galaxie;

    let mut positions: HashMap<u32, Position> = HashMap::new();
    galaxie.iter().enumerate().for_each(|(j, line)| {
        line.iter().enumerate().for_each(|(i, t)| {
            if let Tile::Galaxy(v) = t {
                positions.insert(*v, Position { x: i, y: j });
            }
        })
    });

    let mut sum = 0;

    for i in 1..positions.len() + 1 {
        for j in i..positions.len() + 1 {
            if i != j {
                let mut inner_sum = 0;
                let start = &positions[&(i as u32)];
                let end = &positions[&(j as u32)];

                if start.x > end.x {
                    inner_sum += start.x - end.x;
                } else {
                    inner_sum += end.x - start.x;
                }

                if start.y > end.y {
                    inner_sum += start.y - end.y;
                } else {
                    inner_sum += end.y - start.y;
                }

                // println!("{i} - {j} -> {inner_sum}");
                // println!("{start:?} - {end:?}",);
                sum += inner_sum;
            }
        }
    }

    sum
}

fn part_two(input: &str) -> usize {
    let mut count = 0;
    let galaxie: Vec<Vec<Tile>> = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Space,
                    '#' => {
                        count += 1;
                        Tile::Galaxy(count)
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let lines_to_expand: Vec<usize> = galaxie
        .iter()
        .enumerate()
        .filter_map(|(i, line)| {
            if line.iter().all(|t| *t == Tile::Space) {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    let cols_to_expand: Vec<usize> = galaxie[0]
        .iter()
        .enumerate()
        .filter_map(|(i, c)| {
            if *c == Tile::Space && galaxie.iter().all(|line| line[i] == Tile::Space) {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    let mut positions: HashMap<u32, Position> = HashMap::new();
    galaxie.iter().enumerate().for_each(|(j, line)| {
        line.iter().enumerate().for_each(|(i, t)| {
            if let Tile::Galaxy(v) = t {
                positions.insert(*v, Position { x: i, y: j });
            }
        })
    });

    let mut sum = 0;

    let factor = 1000000 - 1;

    for i in 1..positions.len() + 1 {
        for j in i..positions.len() + 1 {
            if i != j {
                let mut inner_sum = 0;
                let start = &positions[&(i as u32)];
                let end = &positions[&(j as u32)];

                let (min_x, max_x) = if start.x > end.x {
                    (end.x, start.x)
                } else {
                    (start.x, end.x)
                };

                for i in min_x..max_x + 1 {
                    if cols_to_expand.iter().find(|x| **x == i).is_some() {
                        inner_sum += 1 * factor;
                    }
                }

                let (min_y, max_y) = if start.y > end.y {
                    (end.y, start.y)
                } else {
                    (start.y, end.y)
                };

                for j in min_y..max_y + 1 {
                    if lines_to_expand.iter().find(|x| **x == j).is_some() {
                        inner_sum += 1 * factor;
                    }
                }

                inner_sum += max_x - min_x;
                inner_sum += max_y - min_y;

                // println!("{i} - {j} -> {inner_sum}");
                // println!("{start:?} - {end:?}",);
                sum += inner_sum;
            }
        }
    }

    sum
}
