use std::fmt::Debug;

#[macro_export]
macro_rules! aoc_main {
    ($input_file:literal) => {{
        const INPUT_STR: &str = include_str!($input_file);

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
    }};
}

pub fn print_2d_slice<T: Debug>(vec: &[T]) {
    for line in vec {
        println!("{:?}", line);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Default for Position {
    fn default() -> Self {
        Position { x: 0, y: 0 }
    }
}

pub fn get_col<T>(grid: &[Vec<T>], idx: isize) -> Vec<&T> {
    if grid.is_empty() {
        return vec![];
    }

    grid.iter()
        .map(|row| {
            if idx >= 0 {
                &row[idx as usize]
            } else {
                &row[row.len() - idx.abs() as usize]
            }
        })
        .collect()
}

pub fn get_col_mut<T>(grid: &mut [Vec<T>], idx: isize) -> Vec<&mut T> {
    if grid.is_empty() {
        return vec![];
    }

    let row_len = grid[0].len();
    grid.iter_mut()
        .map(|row| {
            if idx >= 0 {
                &mut row[idx as usize]
            } else {
                &mut row[row_len - idx.abs() as usize]
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_col() {
        let t = vec![vec![1, 2], vec![3, 4]];
        let col = get_col(&t, 1 as isize);
        assert_eq!(col, [&2, &4]);
    }

    #[test]
    fn test_get_col_neg_idx() {
        let t = vec![vec![1, 2], vec![3, 4]];
        let col = get_col(&t, -1);
        assert_eq!(col, [&2, &4]);
        let col = get_col(&t, -2);
        assert_eq!(col, [&1, &3]);
    }
}
