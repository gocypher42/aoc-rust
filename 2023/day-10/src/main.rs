use core::fmt;
use std::fmt::Debug;

fn main() {
    const INPUT_STR: &str = include_str!("../inputs/input.txt");

    let start_instant = std::time::Instant::now();
    let part_one_result = part_one(INPUT_STR);
    let part_one_time = start_instant.elapsed().as_secs_f32();

    let start_instant = std::time::Instant::now();
    let part_two_result = part_two(INPUT_STR);
    let part_two_time = start_instant.elapsed().as_secs_f32();

    println!("------");
    println!("AOC {}", env!("CARGO_PKG_NAME"));
    println!("------");
    println!("Part 1 ({:.6} secs): {:?}", part_one_time, part_one_result);
    println!("Part 2 ({:.6} secs): {:?}", part_two_time, part_two_result);
    println!("------");
}

#[derive(Debug, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Default for Position {
    fn default() -> Self {
        Position { x: 0, y: 0 }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    NorthSouth, // '|'
    EastWest,   // '-'
    NorthEast,  // 'L'
    NorthWeast, // 'J'
    SouthWeast, // '7'
    SouthEast,  // 'F'
    Ground,     // '.'
    Start,      // 'S'
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Tile::NorthSouth => '|',
            Tile::EastWest => '-',
            Tile::NorthEast => 'L',
            Tile::NorthWeast => 'J',
            Tile::SouthWeast => '7',
            Tile::SouthEast => 'F',
            Tile::Ground => '.',
            Tile::Start => 'S',
        };
        write!(f, "{}", c)
    }
}

fn print_2d_slice<T: Debug>(vec: &[T]) {
    for line in vec {
        println!("{:?}", line);
    }
}

fn get_tile_from_grid(grid: &Vec<Vec<Tile>>, pos: &Position) -> Tile {
    grid[pos.y][pos.x]
}

fn visited(grid: &Vec<Vec<bool>>, pos: &Position) -> bool {
    grid[pos.y][pos.x]
}

fn can_go_north(
    grid: &Vec<Vec<Tile>>,
    grid_visited: &Vec<Vec<bool>>,
    current_pos: &Position,
) -> Option<Position> {
    if current_pos.y == 0 {
        return None;
    }
    let mut next_pos = current_pos.clone();
    next_pos.y -= 1;
    if visited(&grid_visited, &next_pos) {
        return None;
    }
    if match get_tile_from_grid(grid, &next_pos) {
        Tile::NorthSouth => true,
        Tile::SouthWeast => true,
        Tile::SouthEast => true,
        _ => false,
    } {
        Some(next_pos)
    } else {
        None
    }
}

fn can_go_est(
    grid: &Vec<Vec<Tile>>,
    grid_visited: &Vec<Vec<bool>>,
    current_pos: &Position,
) -> Option<Position> {
    if current_pos.x == grid[0].len() - 1 {
        return None;
    }
    let mut next_pos = current_pos.clone();
    next_pos.x += 1;
    if visited(&grid_visited, &next_pos) {
        return None;
    }
    if match get_tile_from_grid(grid, &next_pos) {
        Tile::NorthWeast => true,
        Tile::SouthWeast => true,
        Tile::EastWest => true,
        _ => false,
    } {
        Some(next_pos)
    } else {
        None
    }
}

fn can_go_south(
    grid: &Vec<Vec<Tile>>,
    grid_visited: &Vec<Vec<bool>>,
    current_pos: &Position,
) -> Option<Position> {
    if current_pos.y == grid.len() - 1 {
        return None;
    }
    let mut next_pos = current_pos.clone();
    next_pos.y += 1;
    if visited(&grid_visited, &next_pos) {
        return None;
    }
    if match get_tile_from_grid(grid, &next_pos) {
        Tile::NorthSouth => true,
        Tile::NorthWeast => true,
        Tile::NorthEast => true,
        _ => false,
    } {
        Some(next_pos)
    } else {
        None
    }
}

fn can_go_west(
    grid: &Vec<Vec<Tile>>,
    grid_visited: &Vec<Vec<bool>>,
    current_pos: &Position,
) -> Option<Position> {
    if current_pos.x == 0 {
        return None;
    }
    let mut next_pos = current_pos.clone();
    next_pos.x -= 1;
    if visited(&grid_visited, &next_pos) {
        return None;
    }
    if match get_tile_from_grid(grid, &next_pos) {
        Tile::EastWest => true,
        Tile::NorthEast => true,
        Tile::SouthEast => true,
        _ => false,
    } {
        Some(next_pos)
    } else {
        None
    }
}

fn get_next_pos(
    grid: &Vec<Vec<Tile>>,
    grid_visited: &Vec<Vec<bool>>,
    pos: &Position,
) -> Option<Position> {
    let next_pos = match get_tile_from_grid(grid, pos) {
        Tile::NorthSouth => {
            if let Some(next_pos) = can_go_north(grid, grid_visited, pos) {
                Some(next_pos)
            } else if let Some(next_pos) = can_go_south(grid, grid_visited, pos) {
                Some(next_pos)
            } else {
                None
            }
        }
        Tile::EastWest => {
            if let Some(next_pos) = can_go_est(grid, grid_visited, pos) {
                Some(next_pos)
            } else if let Some(next_pos) = can_go_west(grid, grid_visited, pos) {
                Some(next_pos)
            } else {
                None
            }
        }
        Tile::NorthEast => {
            if let Some(next_pos) = can_go_north(grid, grid_visited, pos) {
                Some(next_pos)
            } else if let Some(next_pos) = can_go_est(grid, grid_visited, pos) {
                Some(next_pos)
            } else {
                None
            }
        }
        Tile::NorthWeast => {
            if let Some(next_pos) = can_go_north(grid, grid_visited, pos) {
                Some(next_pos)
            } else if let Some(next_pos) = can_go_west(grid, grid_visited, pos) {
                Some(next_pos)
            } else {
                None
            }
        }
        Tile::SouthWeast => {
            if let Some(next_pos) = can_go_south(grid, grid_visited, pos) {
                Some(next_pos)
            } else if let Some(next_pos) = can_go_west(grid, grid_visited, pos) {
                Some(next_pos)
            } else {
                None
            }
        }
        Tile::SouthEast => {
            if let Some(next_pos) = can_go_south(grid, grid_visited, pos) {
                Some(next_pos)
            } else if let Some(next_pos) = can_go_est(grid, grid_visited, pos) {
                Some(next_pos)
            } else {
                None
            }
        }
        Tile::Start => {
            if let Some(next_pos) = can_go_north(grid, grid_visited, pos) {
                Some(next_pos)
            } else if let Some(next_pos) = can_go_south(grid, grid_visited, pos) {
                Some(next_pos)
            } else if let Some(next_pos) = can_go_west(grid, grid_visited, pos) {
                Some(next_pos)
            } else if let Some(next_pos) = can_go_est(grid, grid_visited, pos) {
                Some(next_pos)
            } else {
                None
            }
        }
        Tile::Ground => unreachable!(),
    };
    if let Some(next_pos) = next_pos.clone() {
        println!("{:?}", get_tile_from_grid(grid, &next_pos));
    }
    next_pos
}

fn part_one(input: &str) -> usize {
    let grid: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '|' => Tile::NorthSouth,
                    '-' => Tile::EastWest,
                    'L' => Tile::NorthEast,
                    'J' => Tile::NorthWeast,
                    '7' => Tile::SouthWeast,
                    'F' => Tile::SouthEast,
                    '.' => Tile::Ground,
                    'S' => Tile::Start,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    let mut grid_visited: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];

    let mut current_pos = Position::default();

    let mut steps = 0;

    for (j, line) in grid.iter().enumerate() {
        for (i, cell) in line.iter().enumerate() {
            if *cell == Tile::Start {
                current_pos.x = i;
                current_pos.y = j;
            }
        }
    }
    grid_visited[current_pos.y][current_pos.x] = true;

    // print_2d_slice(&grid);
    // print_2d_slice(&grid_visited);

    println!("initial pos: {:?}", current_pos);

    while let Some(new_pos) = get_next_pos(&grid, &grid_visited, &current_pos) {
        current_pos = new_pos;
        grid_visited[current_pos.y][current_pos.x] = true;
        steps += 1;
        println!("current pos: {:?}", current_pos);
        // print_2d_slice(&grid_visited);
    }

    println!("Steps: {} {}", steps, steps / 2 + 1);

    0
}

fn part_two(input: &str) -> usize {
    0
}
