use std::fmt::Debug;

#[allow(clippy::wildcard_imports)]
use utils::*;

fn main() {
    aoc_main!("../inputs/input.txt");
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Clone)]
enum Tile {
    Ground,
    Obstacle(bool),
    Visited,
    Pipe,
    Dash,
    Cross,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Tile::Ground => write!(f, "."),
            Tile::Obstacle(true) => write!(f, "X"),
            Tile::Obstacle(false) => write!(f, "#"),
            Tile::Visited => write!(f, "X"),
            Tile::Pipe => write!(f, "|"),
            Tile::Dash => write!(f, "-"),
            Tile::Cross => write!(f, "+"),
        }
    }
}

#[derive(Debug, Clone)]
struct Guard {
    direction: Direction,
    position: Position,
}

impl Guard {
    fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        };
    }
}

fn parse_input(input: &str) -> (Vec<Vec<Tile>>, Guard) {
    let mut guard_pos = Guard {
        direction: Direction::Up,
        position: Position::default(),
    };

    let grid: Vec<Vec<Tile>> = input
        .lines()
        .enumerate()
        .map(|(j, l)| {
            l.chars()
                .enumerate()
                .map(|(i, c)| match c {
                    '.' => Tile::Ground,
                    '#' => Tile::Obstacle(false),
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
    return (grid, guard_pos);
}

fn can_move(grid: &Vec<Vec<Tile>>, direction: &Direction, mut pos: Position) -> Option<Position> {
    match direction {
        Direction::Up => {
            if pos.y == 0 {
                return None;
            }
            pos.y -= 1;
        }
        Direction::Down => {
            if pos.y + 1 == grid.len() {
                return None;
            }
            pos.y += 1;
        }
        Direction::Left => {
            if pos.x == 0 {
                return None;
            }
            pos.x -= 1;
        }
        Direction::Right => {
            if pos.x + 1 == grid[0].len() {
                return None;
            }
            pos.x += 1;
        }
    }
    Some(pos)
}

fn part_one(input: &str) -> usize {
    let (mut grid, mut guard_pos) = parse_input(input);

    loop {
        let Some(new_pos) = can_move(&grid, &guard_pos.direction, guard_pos.position.clone())
        else {
            break;
        };
        match grid[new_pos.y][new_pos.x] {
            Tile::Ground | Tile::Visited => {
                guard_pos.position = new_pos;
                grid[guard_pos.position.y][guard_pos.position.x] = Tile::Visited;
            }
            Tile::Obstacle(_) => {
                guard_pos.turn_left();
            }
            _ => {}
        }
    }

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
    let (mut grid, mut guard_pos) = parse_input(input);
    let mut tested = vec![vec![false; grid[0].len()]; grid.len()];
    grid[guard_pos.position.y][guard_pos.position.x] = match guard_pos.direction {
        Direction::Up | Direction::Down => Tile::Pipe,
        Direction::Left | Direction::Right => Tile::Dash,
    };
    let mut count = 0;

    loop {
        let Some(new_pos) = can_move(&grid, &guard_pos.direction, guard_pos.position.clone())
        else {
            break;
        };
        match grid[new_pos.y][new_pos.x] {
            Tile::Cross => {
                guard_pos.position = new_pos;
            }
            Tile::Pipe => {
                guard_pos.position = new_pos;
                match guard_pos.direction {
                    Direction::Left | Direction::Right => {
                        grid[guard_pos.position.y][guard_pos.position.x] = Tile::Cross;
                    }
                    _ => {}
                }
            }
            Tile::Dash => {
                guard_pos.position = new_pos;
                match guard_pos.direction {
                    Direction::Up | Direction::Down => {
                        grid[guard_pos.position.y][guard_pos.position.x] = Tile::Cross;
                    }
                    _ => {}
                }
                grid[guard_pos.position.y][guard_pos.position.x] = Tile::Cross;
            }
            Tile::Ground => {
                if !tested[new_pos.y][new_pos.x] {
                    let mut test_grid_loop = grid.clone();
                    test_grid_loop[new_pos.y][new_pos.x] = Tile::Obstacle(true);
                    let mut test_guard_loop = guard_pos.clone();
                    test_guard_loop.turn_left();
                    if is_loop(&mut test_grid_loop, test_guard_loop) {
                        count += 1;
                    }
                    tested[new_pos.y][new_pos.x] = true;
                }

                guard_pos.position = new_pos;

                match guard_pos.direction {
                    Direction::Up => {
                        grid[guard_pos.position.y][guard_pos.position.x] = Tile::Pipe;
                    }
                    Direction::Down => {
                        grid[guard_pos.position.y][guard_pos.position.x] = Tile::Pipe;
                    }
                    Direction::Left => {
                        grid[guard_pos.position.y][guard_pos.position.x] = Tile::Dash;
                    }
                    Direction::Right => {
                        grid[guard_pos.position.y][guard_pos.position.x] = Tile::Dash;
                    }
                }
            }
            Tile::Obstacle(_) => {
                guard_pos.turn_left();
                grid[guard_pos.position.y][guard_pos.position.x] = Tile::Cross;
            }
            Tile::Visited => unreachable!(),
        }
    }
    count
}

fn is_loop(grid: &mut Vec<Vec<Tile>>, mut guard: Guard) -> bool {
    let mut obs_hit: Vec<(Position, Position)> = vec![];
    loop {
        let Some(new_pos) = can_move(&grid, &guard.direction, guard.position.clone()) else {
            break;
        };

        match grid[new_pos.y][new_pos.x] {
            Tile::Cross => {
                guard.position = new_pos;
            }
            Tile::Pipe => {
                guard.position = new_pos;
                match guard.direction {
                    Direction::Left | Direction::Right => {
                        grid[guard.position.y][guard.position.x] = Tile::Cross;
                    }
                    _ => {}
                }
            }
            Tile::Dash => {
                guard.position = new_pos;
                match guard.direction {
                    Direction::Up | Direction::Down => {
                        grid[guard.position.y][guard.position.x] = Tile::Cross;
                    }
                    _ => {}
                }
                grid[guard.position.y][guard.position.x] = Tile::Cross;
            }
            Tile::Ground => {
                guard.position = new_pos;

                match guard.direction {
                    Direction::Up => {
                        grid[guard.position.y][guard.position.x] = Tile::Pipe;
                    }
                    Direction::Down => {
                        grid[guard.position.y][guard.position.x] = Tile::Pipe;
                    }
                    Direction::Left => {
                        grid[guard.position.y][guard.position.x] = Tile::Dash;
                    }
                    Direction::Right => {
                        grid[guard.position.y][guard.position.x] = Tile::Dash;
                    }
                }
            }
            Tile::Obstacle(plus) => {
                let t = (new_pos.clone(), guard.position.clone());
                if obs_hit.contains(&t) {
                    return true;
                }
                obs_hit.push(t);
                guard.turn_left();
                grid[guard.position.y][guard.position.x] = Tile::Cross;
            }
            Tile::Visited => unreachable!(),
        }
        // print_2d_slice(&grid);
    }
    // print_2d_slice(&grid);
    return false;
}
