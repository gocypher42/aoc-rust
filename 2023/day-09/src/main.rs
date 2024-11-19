fn main() {
    const INPUT_STR: &str = include_str!("../inputs/input.txt");

    let start_instant = std::time::Instant::now();
    let part_one_result = part_one(INPUT_STR);
    let part_one_time = start_instant.elapsed().as_secs_f32();

    let part_two_result = part_two(INPUT_STR);
    let part_two_time = start_instant.elapsed().as_secs_f32() - part_one_time;

    println!("------");
    println!("AOC {}", env!("CARGO_PKG_NAME"));
    println!("------");
    println!("Part 1 ({part_one_time:.6} secs): {part_one_result:?}");
    println!("Part 2 ({part_two_time:.6} secs): {part_two_result:?}");
    println!("------");
}

fn predict_next(input: &[isize]) -> isize {
    if input.iter().all(|item| *item == 0) {
        return 0;
    }

    let jumps: Vec<isize> = input
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect();

    input.last().unwrap() + predict_next(&jumps)
}

fn part_one(input: &str) -> isize {
    input
        .trim()
        .lines()
        .map(|line| {
            let set: Vec<isize> = line
                .split_whitespace()
                .map(|item| item.parse().unwrap())
                .collect();
            predict_next(&set)
        })
        .sum()
}

fn predict_previous(input: &[isize]) -> isize {
    if input.iter().all(|item| *item == 0) {
        return 0;
    }

    let jumps: Vec<isize> = input
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect();

    input.first().unwrap() - predict_previous(&jumps)
}

fn part_two(input: &str) -> isize {
    input
        .trim()
        .lines()
        .map(|line| {
            let set: Vec<isize> = line
                .split_whitespace()
                .map(|item| item.parse().unwrap())
                .collect();
            predict_previous(&set)
        })
        .sum()
}
