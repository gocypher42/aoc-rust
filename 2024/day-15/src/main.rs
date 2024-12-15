#[allow(clippy::wildcard_imports)]
use utils::*;

fn main() {
    aoc_main!("../inputs/input.txt");
}

#[derive(Clone)]
enum Tile {
    Wall,
    Object,
    Space,
    Robot,
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Wall => '#',
                Tile::Object => 'O',
                Tile::Space => '.',
                Tile::Robot => '@',
            }
        )
    }
}

#[derive(Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

fn part_one(input: &str) -> usize {
    let mut grid: Vec<Vec<Tile>> = vec![];
    let mut moves: Vec<Move> = vec![];
    {
        let mut in_dir = false;
        for line in input.lines() {
            if line.is_empty() {
                in_dir = true;
                continue;
            }

            if in_dir {
                for c in line.chars() {
                    moves.push(match c {
                        '<' => Move::Left,
                        '^' => Move::Up,
                        '>' => Move::Right,
                        'v' => Move::Down,
                        _ => unreachable!(),
                    })
                }
                continue;
            }
            grid.push(
                line.chars()
                    .map(|c| match c {
                        '#' => Tile::Wall,
                        '.' => Tile::Space,
                        '@' => Tile::Robot,
                        'O' => Tile::Object,
                        _ => unreachable!(),
                    })
                    .collect(),
            );
        }
    }
    let moves = moves;
    let mut robot_pos = Position::default();
    for j in 0..grid.len() {
        for i in 0..grid[0].len() {
            if matches!(grid[j][i], Tile::Robot) {
                robot_pos.x = i;
                robot_pos.y = j;
                break;
            }
        }
    }

    print_2d_slice(&grid);
    print_2d_slice(&moves);
    println!("{robot_pos:?}");

    for robot_move in moves {
        println!("--- {robot_move:?}");
        match robot_move {
            Move::Up => {
                let s = &mut get_col_mut(&mut grid, robot_pos.x as isize)[0..robot_pos.y];

                let Some(space_pos) = s
                    .iter()
                    .enumerate()
                    .rev()
                    .find(|(_, cell)| matches!(cell, Tile::Space))
                    .map(|(i, _)| i)
                else {
                    continue;
                };

                let wall_pos = s
                    .iter()
                    .enumerate()
                    .rev()
                    .find(|(_, cell)| matches!(cell, Tile::Wall))
                    .map(|(i, _)| i)
                    .unwrap();

                println!("{s:?} free: {space_pos} wall: {wall_pos}");

                if space_pos > wall_pos {
                    for i in space_pos..s.len() - 1 {
                        *s[i] = s[i + 1].clone();
                    }

                    grid[robot_pos.y][robot_pos.x] = Tile::Space;
                    robot_pos.y -= 1;
                    grid[robot_pos.y][robot_pos.x] = Tile::Robot;
                }
            }
            Move::Down => {
                let s = &mut get_col_mut(&mut grid, robot_pos.x as isize)[robot_pos.y + 1..];

                let Some(space_pos) = s.iter().position(|cell| matches!(cell, Tile::Space)) else {
                    continue;
                };

                let wall_pos = s
                    .iter()
                    .position(|cell| matches!(cell, Tile::Wall))
                    .unwrap();

                println!("{s:?} free: {space_pos} wall: {wall_pos}");

                if space_pos < wall_pos {
                    for i in robot_pos.y + 1..robot_pos.y + 1 + space_pos {
                        grid[i + 1][robot_pos.x] = grid[i][robot_pos.x].clone();
                    }
                    grid[robot_pos.y][robot_pos.x] = Tile::Space;
                    robot_pos.y += 1;
                    grid[robot_pos.y][robot_pos.x] = Tile::Robot;
                }
            }
            Move::Left => {
                let s = &mut grid[robot_pos.y][0..robot_pos.x].to_vec();

                let Some(space_pos) = s
                    .iter()
                    .enumerate()
                    .rev()
                    .find(|(_, cell)| matches!(cell, Tile::Space))
                    .map(|(i, _)| i)
                else {
                    continue;
                };

                let wall_pos = s
                    .iter()
                    .enumerate()
                    .rev()
                    .find(|(_, cell)| matches!(cell, Tile::Wall))
                    .map(|(i, _)| i)
                    .unwrap();

                println!("{s:?} free: {space_pos} wall: {wall_pos}");

                if space_pos > wall_pos {
                    for i in space_pos..s.len() - 1 {
                        grid[robot_pos.y][i] = grid[robot_pos.y][i + 1].clone();
                    }
                    grid[robot_pos.y][robot_pos.x] = Tile::Space;
                    robot_pos.x -= 1;
                    grid[robot_pos.y][robot_pos.x] = Tile::Robot;
                }
            }
            Move::Right => {
                let s = grid[robot_pos.y][robot_pos.x + 1..].to_vec();

                let Some(space_pos) = s.iter().position(|cell| matches!(cell, Tile::Space)) else {
                    continue;
                };

                let wall_pos = s
                    .iter()
                    .position(|cell| matches!(cell, Tile::Wall))
                    .unwrap();

                println!("{s:?} free: {space_pos} wall: {wall_pos}");

                if space_pos < wall_pos {
                    println!("Try move right");
                    for i in robot_pos.x + 1..robot_pos.x + 1 + space_pos {
                        grid[robot_pos.y][i + 1] = grid[robot_pos.y][i].clone();
                    }
                    grid[robot_pos.y][robot_pos.x] = Tile::Space;
                    robot_pos.x += 1;
                    grid[robot_pos.y][robot_pos.x] = Tile::Robot;
                }
            }
        }
        // print_2d_slice(&grid);
    }

    grid.iter()
        .enumerate()
        .flat_map(|(j, row)| {
            row.iter().enumerate().filter_map(move |(i, cell)| {
                if matches!(cell, Tile::Object) {
                    let v = (100 * j) + i;
                    println!("100 * {j} + {i} = {v}");
                    Some(v)
                } else {
                    None
                }
            })
        })
        .sum()
}

fn part_two(input: &str) -> usize {
    0
}
