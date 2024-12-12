use std::collections::HashMap;

#[allow(clippy::wildcard_imports)]
use utils::*;

fn main() {
    aoc_main!("../inputs/input.txt");
}

#[derive(Clone)]
struct Tile {
    seed: char,
    patch: usize,
    edges: usize,
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.seed, self.patch, self.edges)
    }
}

fn part_one(input: &str) -> usize {
    let mut grid: Vec<Vec<Tile>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| Tile {
                    seed: c,
                    patch: 0,
                    edges: 0,
                })
                .collect()
        })
        .collect();
    // print_2d_slice(&grid);

    let max_j = grid.len() - 1;
    let max_i = grid[0].len() - 1;

    for j in 0..grid.len() {
        for i in 0..grid[0].len() {
            let mut cell = grid[j][i].clone();
            if i == 0 {
                cell.edges += 1;
            }
            if i == max_i {
                cell.edges += 1;
            }
            if j == 0 {
                cell.edges += 1;
            }
            if j == max_j {
                cell.edges += 1;
            }

            if let Some(a_cell) = grid.get(j).and_then(|row| row.get(i + 1)) {
                if a_cell.seed != cell.seed {
                    cell.edges += 1;
                }
            }
            if let Some(a_cell) = grid
                .get(j)
                .and_then(|row| if i > 0 { row.get(i - 1) } else { None })
            {
                if a_cell.seed != cell.seed {
                    cell.edges += 1;
                }
            }
            if let Some(a_cell) = grid.get(j + 1).and_then(|row| row.get(i)) {
                if a_cell.seed != cell.seed {
                    cell.edges += 1;
                }
            }
            if let Some(a_cell) = if j > 0 {
                grid.get(j - 1).and_then(|row| row.get(i))
            } else {
                None
            } {
                if a_cell.seed != cell.seed {
                    cell.edges += 1;
                }
            }
            grid[j][i] = cell;
        }
    }

    // println!();
    // print_2d_slice(&grid);
    let mut next_id = 1;

    for j in 0..grid.len() {
        for i in 0..grid[0].len() {
            if grid[j][i].patch != 0 {
                continue;
            }
            let c = grid[j][i].seed;
            make_plot(&mut grid, i, j, c, next_id);
            next_id += 1;
        }
    }

    // println!();
    // print_2d_slice(&grid);
    let mut garden: HashMap<(char, usize), (usize, usize)> = HashMap::new();

    for row in &grid {
        for cell in row {
            garden.entry((cell.seed, cell.patch)).or_insert((0, 0)).0 += 1;
            garden.entry((cell.seed, cell.patch)).or_insert((0, 0)).1 += cell.edges;
        }
    }

    // println!();
    // for k in &garden {
    //     println!("{k:?}");
    // }
    garden
        .iter()
        .map(|(_, (plot_size, edges))| plot_size * edges)
        .sum()
}

fn make_plot(grid: &mut Vec<Vec<Tile>>, i: usize, j: usize, c: char, id: usize) {
    if i >= grid[0].len() || j >= grid.len() {
        return;
    }

    if grid[j][i].seed != c || grid[j][i].patch != 0 {
        return;
    }

    grid[j][i].patch = id;

    if i > 0 {
        make_plot(grid, i - 1, j, c, id);
    }
    if i < grid[0].len() {
        make_plot(grid, i + 1, j, c, id);
    }

    if j > 0 {
        make_plot(grid, i, j - 1, c, id);
    }

    if j < grid.len() {
        make_plot(grid, i, j + 1, c, id);
    }
}

fn part_two(input: &str) -> usize {
    let def: [Vec<Position>; 4] = [vec![], vec![], vec![], vec![]];

    let mut grid: Vec<Vec<Tile>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| Tile {
                    seed: c,
                    patch: 0,
                    edges: 0,
                })
                .collect()
        })
        .collect();

    let mut next_id = 1;
    for j in 0..grid.len() {
        for i in 0..grid[0].len() {
            if grid[j][i].patch != 0 {
                continue;
            }
            let c = grid[j][i].seed;
            make_plot(&mut grid, i, j, c, next_id);
            next_id += 1;
        }
    }

    let max_j = grid.len() - 1;
    let max_i = grid[0].len() - 1;

    let mut garden: HashMap<(char, usize), [Vec<Position>; 4]> = HashMap::new();

    for j in 0..grid.len() {
        for i in 0..grid[0].len() {
            let cell = &grid[j][i];

            if i == 0 {
                garden.entry((cell.seed, cell.patch)).or_insert(def.clone())[0]
                    .push(Position { x: i, y: j });
            }
            if i == max_i {
                garden.entry((cell.seed, cell.patch)).or_insert(def.clone())[3]
                    .push(Position { x: i, y: j });
            }
            if j == 0 {
                garden.entry((cell.seed, cell.patch)).or_insert(def.clone())[2]
                    .push(Position { x: i, y: j });
            }
            if j == max_j {
                garden.entry((cell.seed, cell.patch)).or_insert(def.clone())[1]
                    .push(Position { x: i, y: j });
            }

            if let Some(a_cell) = grid.get(j).and_then(|row| row.get(i + 1)) {
                if a_cell.seed != cell.seed {
                    garden.entry((cell.seed, cell.patch)).or_insert(def.clone())[3]
                        .push(Position { x: i, y: j });
                }
            }
            if let Some(a_cell) = grid
                .get(j)
                .and_then(|row| if i > 0 { row.get(i - 1) } else { None })
            {
                if a_cell.seed != cell.seed {
                    garden.entry((cell.seed, cell.patch)).or_insert(def.clone())[0]
                        .push(Position { x: i, y: j });
                }
            }
            if let Some(a_cell) = grid.get(j + 1).and_then(|row| row.get(i)) {
                if a_cell.seed != cell.seed {
                    garden.entry((cell.seed, cell.patch)).or_insert(def.clone())[1]
                        .push(Position { x: i, y: j });
                }
            }
            if let Some(a_cell) = if j > 0 {
                grid.get(j - 1).and_then(|row| row.get(i))
            } else {
                None
            } {
                if a_cell.seed != cell.seed {
                    garden.entry((cell.seed, cell.patch)).or_insert(def.clone())[2]
                        .push(Position { x: i, y: j });
                }
            }
        }
    }

    let mut garden_count: HashMap<(char, usize), (usize, usize)> = HashMap::new();

    for row in &grid {
        for cell in row {
            garden_count
                .entry((cell.seed, cell.patch))
                .or_insert((0, 0))
                .0 += 1;
        }
    }

    for (key, dirr_arr) in garden {
        let mut sides = 0;
        sides += check_wall_y(&dirr_arr[0]);
        sides += check_wall_y(&dirr_arr[3]);
        sides += check_wall_x(&dirr_arr[1]);
        sides += check_wall_x(&dirr_arr[2]);
        garden_count.entry(key).or_insert((0, 0)).1 = sides;
    }

    garden_count
        .iter()
        .map(|(_, (plot_size, edges))| plot_size * edges)
        .sum()
}

fn check_wall_x(dir: &[Position]) -> usize {
    let mut sides = 0;
    let mut seen: Vec<Position> = vec![];
    for pos in dir {
        if seen.contains(pos) {
            continue;
        }
        sides += 1;

        seen.push(pos.clone());

        let mut s_pos = pos.clone();

        while s_pos.x > 0 {
            s_pos.x -= 1;
            if dir.contains(&s_pos) && !seen.contains(&s_pos) {
                seen.push(s_pos.clone());
            }
        }

        s_pos.x = pos.x;
        s_pos.x += 1;
        while dir.contains(&s_pos) {
            if !seen.contains(&s_pos) {
                seen.push(s_pos.clone());
            }
            s_pos.x += 1;
        }
    }
    sides
}

fn check_wall_y(dir: &[Position]) -> usize {
    let mut sides = 0;
    let mut seen: Vec<Position> = vec![];
    for pos in dir {
        if seen.contains(pos) {
            continue;
        }
        sides += 1;

        seen.push(pos.clone());

        let mut s_pos = pos.clone();

        while s_pos.y > 0 {
            s_pos.y -= 1;
            if dir.contains(&s_pos) && !seen.contains(&s_pos) {
                seen.push(s_pos.clone());
            }
        }

        s_pos.y = pos.y;
        s_pos.y += 1;
        while dir.contains(&s_pos) {
            if !seen.contains(&s_pos) {
                seen.push(s_pos.clone());
            }
            s_pos.y += 1;
        }
    }
    sides
}
