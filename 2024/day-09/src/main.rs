#[allow(clippy::wildcard_imports)]
use utils::*;

fn main() {
    aoc_main!("../inputs/input.txt");
}

#[derive(Debug)]
enum Block {
    Free { size: u32 },
    File { id: usize, size: u32 },
}

fn part_one(input: &str) -> usize {
    let mut is_file = true;
    let mut id = 0;
    let mut blocks: Vec<Block> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|v| {
            let block = if is_file {
                let b = Block::File { id, size: v };
                id += 1;
                b
            } else {
                Block::Free { size: v }
            };
            is_file = !is_file;
            block
        })
        .filter(|b| {
            let s = match *b {
                Block::Free { size } => size,
                Block::File { size, .. } => size,
            };
            s != 0
        })
        .collect();

    loop {
        if let Block::Free { .. } = blocks.last().unwrap() {
            blocks.pop();
            continue;
        }

        if let Block::File { id, size } = blocks.last().unwrap() {
            let mut size_to_swap: u32 = 0;
            let free_space_block = match blocks.iter().position(|x| match x {
                Block::Free { size: free_size } => {
                    if *free_size != 0 {
                        size_to_swap = std::cmp::min(*size, *free_size);

                        return true;
                    }
                    false
                }
                Block::File { .. } => false,
            }) {
                Some(p) => p,
                None => break,
            };

            blocks.insert(
                free_space_block,
                Block::File {
                    id: *id,
                    size: size_to_swap,
                },
            );

            if let Block::Free { size } = blocks.get_mut(free_space_block + 1).unwrap() {
                *size -= size_to_swap;
            }

            if let Block::File { size, .. } = blocks.last_mut().unwrap() {
                *size -= size_to_swap;
                if *size == 0 {
                    blocks.pop();
                }
            }
        }
    }

    blocks = blocks
        .into_iter()
        .filter(|x| match x {
            Block::Free { size } => *size != 0,
            Block::File { .. } => true,
        })
        .collect();

    let mut id = 0;
    blocks
        .iter()
        .map(|x| match x {
            Block::File { id: file_id, size } => (0..*size)
                .map(|_| {
                    let y = id * file_id;
                    id += 1;
                    y
                })
                .sum(),
            _ => 0,
        })
        .sum()
}

fn part_two(input: &str) -> usize {
    let mut is_file = true;
    let mut id = 0;
    let mut blocks: Vec<Block> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|v| {
            let block = if is_file {
                let b = Block::File { id, size: v };
                id += 1;
                b
            } else {
                Block::Free { size: v }
            };
            is_file = !is_file;
            block
        })
        .filter(|b| {
            let s = match *b {
                Block::Free { size } => size,
                Block::File { size, .. } => size,
            };
            s != 0
        })
        .collect();
    print_2d_slice(&blocks);
    println!();

    loop {

        if let Block::Free { .. } = blocks.last().unwrap() {
            blocks.pop();
            continue;
        }

        if let Block::File { id, size } = blocks.last().unwrap() {
            let size_to_swap = *size;
            let free_space_block = match blocks.iter().position(|x| match x {
                Block::Free { size: free_size } => {
                    if free_size >= size {
                        return true;
                    }
                    false
                }
                Block::File { .. } => false,
            }) {
                Some(p) => p,
                None => break,
            };

            blocks.insert(
                free_space_block,
                Block::File {
                    id: *id,
                    size: size_to_swap,
                },
            );

            if let Block::Free { size: free_size } = blocks.get_mut(free_space_block + 1).unwrap() {
                *free_size -= size_to_swap;
            }

            //
            // if let Block::File { size, .. } = blocks.last_mut().unwrap() {
            //     *size -= size_to_swap;
            //     if *size == 0 {
            //         blocks.pop();
            //     }
            // }

            break;
        }
    }
    print_2d_slice(&blocks);

    let mut id = 0;
    blocks
        .iter()
        .map(|x| match x {
            Block::File { id: file_id, size } => (0..*size)
                .map(|_| {
                    let y = id * file_id;
                    id += 1;
                    y
                })
                .sum::<usize>(),
            Block::Free { size } => (0..*size)
                .map(|_| {
                    let y = id * 0;
                    id += 1;
                    y
                })
                .sum(),
        })
        .sum()
}
