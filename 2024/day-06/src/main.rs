use std::fmt::Debug;

#[allow(clippy::wildcard_imports)]
use utils::*;

fn main() {
    aoc_main!("../inputs/input.txt");
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq)]
enum Tile {
    Ground,
    Obstacle,
    Visited,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Tile::Ground => write!(f, "."),
            Tile::Obstacle => write!(f, "#"),
            Tile::Visited => write!(f, "X"),
        }
    }
}

#[derive(Debug)]
struct Guard {
    direction: Direction,
    position: Position,
}

fn part_one(input: &str) -> usize {
    println!("{input}");

    let mut guard_pos = Guard {
        direction: Direction::Up,
        position: Position::default(),
    };

    let mut grid: Vec<Vec<Tile>> = input
        .lines()
        .enumerate()
        .map(|(j, l)| {
            l.chars()
                .enumerate()
                .map(|(i, c)| match c {
                    '.' => Tile::Ground,
                    '#' => Tile::Obstacle,
                    '^' => {
                        guard_pos = Guard {
                            direction: Direction::Up,
                            position: Position { x: i, y: j },
                        };
                        Tile::Visited
                    }
                    'v' => {
                        guard_pos = Guard {
                            direction: Direction::Down,
                            position: Position { x: i, y: j },
                        };
                        Tile::Visited
                    }
                    '<' => {
                        guard_pos = Guard {
                            direction: Direction::Left,
                            position: Position { x: i, y: j },
                        };
                        Tile::Visited
                    }
                    '>' => {
                        guard_pos = Guard {
                            direction: Direction::Right,
                            position: Position { x: i, y: j },
                        };
                        Tile::Visited
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    // print_2d_slice(&grid);
    //
    // println!("{guard_pos:?}");

    loop {
        let mut new_pos = guard_pos.position.clone();
        match guard_pos.direction {
            Direction::Up => {
                if new_pos.y == 0 {
                    break;
                }
                new_pos.y -= 1;
            }
            Direction::Down => {
                if new_pos.y + 1 == grid.len() {
                    break;
                }
                new_pos.y += 1;
            }
            Direction::Left => {
                if new_pos.x == 0 {
                    break;
                }
                new_pos.x -= 1;
            }
            Direction::Right => {
                if new_pos.x + 1 == grid[0].len() {
                    break;
                }
                new_pos.x += 1;
            }
        }

        match grid[new_pos.y][new_pos.x] {
            Tile::Ground | Tile::Visited => {
                guard_pos.position = new_pos;
                grid[guard_pos.position.y][guard_pos.position.x] = Tile::Visited;
            }
            Tile::Obstacle => {
                guard_pos.direction = match guard_pos.direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                };
            }
        }
    }
    // print_2d_slice(&grid);

    grid.iter()
        .flat_map(|row| {
            row.iter().filter_map(|cell| {
                if *cell == Tile::Visited {
                    Some(())
                } else {
                    None
                }
            })
        })
        .count()
}

fn part_two(input: &str) -> usize {
    println!("{input}");

    let mut guard_pos = Guard {
        direction: Direction::Up,
        position: Position::default(),
    };

    let mut grid: Vec<Vec<Tile>> = input
        .lines()
        .enumerate()
        .map(|(j, l)| {
            l.chars()
                .enumerate()
                .map(|(i, c)| match c {
                    '.' => Tile::Ground,
                    '#' => Tile::Obstacle,
                    '^' => {
                        guard_pos = Guard {
                            direction: Direction::Up,
                            position: Position { x: i, y: j },
                        };
                        Tile::Visited
                    }
                    'v' => {
                        guard_pos = Guard {
                            direction: Direction::Down,
                            position: Position { x: i, y: j },
                        };
                        Tile::Visited
                    }
                    '<' => {
                        guard_pos = Guard {
                            direction: Direction::Left,
                            position: Position { x: i, y: j },
                        };
                        Tile::Visited
                    }
                    '>' => {
                        guard_pos = Guard {
                            direction: Direction::Right,
                            position: Position { x: i, y: j },
                        };
                        Tile::Visited
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    // print_2d_slice(&grid);
    //
    // println!("{guard_pos:?}");

    loop {
        let mut new_pos = guard_pos.position.clone();
        match guard_pos.direction {
            Direction::Up => {
                if new_pos.y == 0 {
                    break;
                }
                new_pos.y -= 1;
            }
            Direction::Down => {
                if new_pos.y + 1 == grid.len() {
                    break;
                }
                new_pos.y += 1;
            }
            Direction::Left => {
                if new_pos.x == 0 {
                    break;
                }
                new_pos.x -= 1;
            }
            Direction::Right => {
                if new_pos.x + 1 == grid[0].len() {
                    break;
                }
                new_pos.x += 1;
            }
        }

        match grid[new_pos.y][new_pos.x] {
            Tile::Visited => {
                match guard_pos.direction {
                    Direction::Up => {
                        if new_pos.x + 1 < grid[0].len() {
                            if grid[new_pos.y][new_pos.x + 1] == Tile::Visited {
                                println!("Uloop");
                            }
                        }
                    }
                    Direction::Down => {
                        if new_pos.x != 0 {
                            if grid[new_pos.y][new_pos.x - 1] == Tile::Visited {
                                println!("Dloop");
                            }
                        }
                    }
                    Direction::Left => {
                        if new_pos.y != 0 {
                            if grid[new_pos.y - 1][new_pos.x] == Tile::Visited {
                                println!("Lloop");
                            }
                        }
                    }
                    Direction::Right => {
                        if new_pos.y < grid.len() {
                            if grid[new_pos.y + 1][new_pos.x] == Tile::Visited {
                                println!("Rloop");
                            }
                        }
                    }
                };
            }
            _ => {}
        }

        match grid[new_pos.y][new_pos.x] {
            Tile::Ground | Tile::Visited => {
                guard_pos.position = new_pos;
                grid[guard_pos.position.y][guard_pos.position.x] = Tile::Visited;
            }
            Tile::Obstacle => {
                // print_2d_slice(&grid);
                // println!("--");
                guard_pos.direction = match guard_pos.direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                };
            }
        }
    }
    // print_2d_slice(&grid);

    grid.iter()
        .flat_map(|row| {
            row.iter().filter_map(|cell| {
                if *cell == Tile::Visited {
                    Some(())
                } else {
                    None
                }
            })
        })
        .count()
}
