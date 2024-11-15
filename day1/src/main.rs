fn main() {
    const INPUT_STR: &str = include_str!("../inputs/input.txt");

    let part_one_result = part_one(INPUT_STR);
    let part_two_result = part_two(INPUT_STR);

    println!("AOC Day 1");
    println!(" ---- ");
    println!("Part 1: {:?}", part_one_result);
    println!("Part 2: {:?}", part_two_result);
    println!(" ---- ");
}

fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split('\n')
            .filter(|v| !v.is_empty())
            .map(|l| {
                let mut first = 0;
                let mut last = 0;
                for c in l.chars() {
                    if let Some(v) = c.to_digit(10) {
                        first = v;
                        break;
                    }
                }
                for c in l.chars().rev() {
                    if let Some(v) = c.to_digit(10) {
                        last = v;
                        break;
                    }
                }
                first * 10 + last
            })
            .sum(),
    )
}

fn part_two(input: &str) -> Option<u32> {
    None
}
