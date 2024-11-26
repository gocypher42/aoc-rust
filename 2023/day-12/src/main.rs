use utils::*;

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

fn part_one(input: &str) -> usize {
0

}

fn part_two(input: &str) -> usize {
 0
}
