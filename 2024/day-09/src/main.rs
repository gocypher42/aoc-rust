#[allow(clippy::wildcard_imports)]
use utils::*;

fn main() {
    aoc_main!("../inputs/input.txt");
}

#[derive(Debug, Clone)]
enum Block {
    Free { size: u32 },
    File { id: usize, size: u32 },
}

fn parse_input(input: &str) -> Vec<Block> {
    let mut is_file = true;
    let mut id = 0;
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|size| {
            let block = if is_file {
                Block::File { id, size }
            } else {
                Block::Free { size }
            };
            id += usize::from(is_file);
            is_file = !is_file;
            block
        })
        .filter(|b| match *b {
            Block::Free { size } | Block::File { size, .. } => size != 0,
        })
        .collect()
}

fn part_one(input: &str) -> usize {
    let mut blocks = parse_input(input);

    loop {
        while matches!(blocks.last(), Some(Block::Free { .. })) {
            blocks.pop();
        }

        if let Some(Block::File { id, size }) = blocks.last().cloned() {
            let mut size_to_swap: u32 = 0;

            let free_space_block = blocks.iter().position(|block| match block {
                Block::Free { size: free_size } => {
                    size_to_swap = size.min(*free_size);
                    *free_size > 0
                }
                Block::File { .. } => false,
            });

            if let Some(pos) = free_space_block {
                blocks.insert(
                    pos,
                    Block::File {
                        id,
                        size: size_to_swap,
                    },
                );

                if let Block::Free { size } = &mut blocks[pos + 1] {
                    *size -= size_to_swap;
                }

                if let Some(Block::File { size, .. }) = blocks.last_mut() {
                    *size -= size_to_swap;
                    if *size == 0 {
                        blocks.pop();
                    }
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }

    let mut id = 0;
    blocks
        .iter()
        .map(|block| match block {
            Block::File { id: file_id, size } => (0..*size)
                .map(|_| {
                    let value = id * file_id;
                    id += 1;
                    value
                })
                .sum(),
            Block::Free { .. } => 0,
        })
        .sum()
}

fn part_two(input: &str) -> usize {
    let mut blocks = parse_input(input);

    let mut files_to_skip = 0;
    loop {
        let file_block = blocks
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, block)| matches!(block, Block::File { .. }))
            .nth(files_to_skip);

        let Some((
            pos,
            Block::File {
                id,
                size: file_size,
            },
        )) = file_block
        else {
            break;
        };

        let swap = *file_size;

        let free_block = blocks
            .iter()
            .enumerate()
            .take(pos)
            .find(|(_, block)| matches!(block, Block::Free { size } if *size >= swap));

        let Some((free_pos, Block::Free { .. })) = free_block else {
            files_to_skip += 1;
            continue;
        };

        blocks.insert(
            free_pos,
            Block::File {
                id: *id,
                size: swap,
            },
        );

        if let Block::Free { size } = blocks.get_mut(free_pos + 1).unwrap() {
            *size -= swap;
        }

        *blocks.get_mut(pos + 1).unwrap() = Block::Free { size: swap };
    }

    let mut id = 0;
    blocks
        .iter()
        .map(|x| match x {
            Block::File { id: file_id, size } => (0..*size)
                .map(|_| {
                    let value = id * file_id;
                    id += 1;
                    value
                })
                .sum::<usize>(),
            Block::Free { size } => (0..*size)
                .map(|_| {
                    id += 1;
                    0
                })
                .sum(),
        })
        .sum()
}
