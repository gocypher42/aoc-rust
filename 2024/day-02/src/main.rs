use utils::*;

fn main() {
    aoc_main!("../inputs/input.txt");
}

fn part_one(input: &str) -> usize {
    input
        .lines()
        .filter_map(|l| {
            let nums: Vec<usize> = l.split(" ").map(|c| c.parse().unwrap()).collect();
            if is_safe(&nums) {
                Some(())
            } else {
                None
            }
        })
        .count()
}

fn part_two(input: &str) -> usize {
    input
        .lines()
        .filter_map(|l| {
            let nums: Vec<usize> = l.split(" ").map(|c| c.parse().unwrap()).collect();
            if is_safe(&nums) {
                return Some(());
            }

            for i in 0..nums.len() {
                let mut sub_nums = nums.clone();
                sub_nums.remove(i);
                if is_safe(&sub_nums) {
                    return Some(());
                }
            }
            // println!("{l}");

            None
        })
        .count()
}

#[derive(Debug)]
enum Direction {
    Inc,
    Dec,
}

fn is_safe(nums: &[usize]) -> bool {
    let direction = if nums[0] > nums[1] {
        Direction::Dec
    } else {
        Direction::Inc
    };

    for i in 0..nums.len() - 1 {
        match direction {
            Direction::Inc => {
                if nums[i] > nums[i + 1] {
                    return false;
                }
            }
            Direction::Dec => {
                if nums[i] < nums[i + 1] {
                    return false;
                }
            }
        }

        let diff = nums[i].abs_diff(nums[i + 1]);

        if diff < 1 || diff > 3 {
            return false;
        }
    }

    true
}
