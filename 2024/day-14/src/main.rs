use std::{collections::HashMap, isize};

#[allow(clippy::wildcard_imports)]
use utils::aoc_main;
use utils::print_2d_slice;

fn main() {
    aoc_main!("../inputs/input.txt");
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Position {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Velocity {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Robot {
    position: Position,
    velocity: Velocity,
}

fn part_one(input: &str) -> usize {
    const GRID_WIDTH: u8 = 101;
    const GRID_HEIGHT: u8 = 103;

    let mut robots: Vec<Robot> = vec![];

    for line in input.lines() {
        let (pos, vel) = line.split_once(' ').unwrap();
        let (px, py) = pos[2..].split_once(',').unwrap();
        let (vx, vy) = vel[2..].split_once(',').unwrap();
        robots.push(Robot {
            position: Position {
                x: px.parse().unwrap(),
                y: py.parse().unwrap(),
            },
            velocity: Velocity {
                x: vx.parse().unwrap(),
                y: vy.parse().unwrap(),
            },
        });
    }

    const SECONDS: u8 = 100;

    for _ in 0..SECONDS {
        for robot in robots.iter_mut() {
            robot.position.x += robot.velocity.x;
            robot.position.y += robot.velocity.y;

            if robot.position.y < 0 {
                robot.position.y = GRID_HEIGHT as isize - robot.position.y.abs();
            }

            if robot.position.y >= GRID_HEIGHT as isize {
                robot.position.y -= GRID_HEIGHT as isize;
            }

            if robot.position.x < 0 {
                robot.position.x = GRID_WIDTH as isize - robot.position.x.abs();
            }

            if robot.position.x >= GRID_WIDTH as isize {
                robot.position.x -= GRID_WIDTH as isize;
            }
        }
    }

    let width_middle = GRID_WIDTH / 2;
    let height_middle = GRID_HEIGHT / 2;

    let mut quadrant_count = [0; 4];

    for robot in robots {
        if robot.position.x < width_middle as isize && robot.position.y < height_middle as isize {
            quadrant_count[0] += 1;
        }

        if robot.position.x > width_middle as isize && robot.position.y < height_middle as isize {
            quadrant_count[1] += 1;
        }

        if robot.position.x < width_middle as isize && robot.position.y > height_middle as isize {
            quadrant_count[2] += 1;
        }

        if robot.position.x > width_middle as isize && robot.position.y > height_middle as isize {
            quadrant_count[3] += 1;
        }
    }

    quadrant_count[0] * quadrant_count[1] * quadrant_count[2] * quadrant_count[3]
}

fn part_two(input: &str) -> usize {
    const GRID_WIDTH: u8 = 101;
    const GRID_HEIGHT: u8 = 103;

    let mut robots: Vec<Robot> = vec![];

    for line in input.lines() {
        if line[..1] == *"#" {
            continue;
        }
        let (pos, vel) = line.split_once(' ').unwrap();
        let (px, py) = pos[2..].split_once(',').unwrap();
        let (vx, vy) = vel[2..].split_once(',').unwrap();
        robots.push(Robot {
            position: Position {
                x: px.parse().unwrap(),
                y: py.parse().unwrap(),
            },
            velocity: Velocity {
                x: vx.parse().unwrap(),
                y: vy.parse().unwrap(),
            },
        });
    }

    let mut second = 0;
    loop {
        for robot in robots.iter_mut() {
            robot.position.x += robot.velocity.x;
            robot.position.y += robot.velocity.y;

            if robot.position.y < 0 {
                robot.position.y = GRID_HEIGHT as isize - robot.position.y.abs();
            }

            if robot.position.y >= GRID_HEIGHT as isize {
                robot.position.y -= GRID_HEIGHT as isize;
            }

            if robot.position.x < 0 {
                robot.position.x = GRID_WIDTH as isize - robot.position.x.abs();
            }

            if robot.position.x >= GRID_WIDTH as isize {
                robot.position.x -= GRID_WIDTH as isize;
            }
        }

        let mut grid = vec![vec!['.'; GRID_WIDTH.into()]; GRID_HEIGHT.into()];

        for robot in &robots {
            grid[robot.position.y as usize][robot.position.x as usize] = '#';
        }

        for row in &grid {
            if row
                .windows(30)
                .into_iter()
                .filter(|win| win.iter().all(|x| *x == '#'))
                .count()
                >= 2
            {
                return second + 1;
            }
        }
        second += 1;
    }
}
