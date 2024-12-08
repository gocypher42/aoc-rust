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

