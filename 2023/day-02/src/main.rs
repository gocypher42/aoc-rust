fn main() {
    const INPUT_STR: &str = include_str!("../inputs/input.txt");

    let start_instant = std::time::Instant::now();
    let part_one_result = part_one(INPUT_STR);
    let part_one_time = start_instant.elapsed().as_secs_f32();

    let part_two_result = part_two(INPUT_STR);
    let part_two_time = start_instant.elapsed().as_secs_f32() - part_one_time;

    println!("------");
    println!("AOC Day 2");
    println!("------");
    println!("Part 1 ({part_one_time}): {part_one_result:?}");
    println!("Part 2 ({part_two_time}): {part_two_result:?}");
    println!("------");

    let t = std::time::Instant::now();
    let part1_2 = part_one_2_0();
    let t_r = t.elapsed().as_secs_f32();
    println!("------");
    println!("Part 1 ({t_r}): {part1_2:?}");
    println!("------");
}

const R_CUBES: u32 = 12;
const G_CUBES: u32 = 13;
const B_CUBES: u32 = 14;

fn part_one(input: &str) -> u32 {
    let mut sum = 0;
    for game in input.lines() {
        let id_and_pulls: Vec<&str> = game.split(':').collect();

        let id: u32 = id_and_pulls[0].split(' ').collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();

        let mut game_is_possible = true;

        for pull in id_and_pulls[1].split(';') {
            let mut rgb = [0, 0, 0];

            for color in pull.split(',') {
                let color_parts: Vec<&str> = color.trim().split(' ').collect();
                let cubes = color_parts[0].parse::<u32>().unwrap();

                let i = match color_parts[1] {
                    "red" => 0,
                    "green" => 1,
                    "blue" => 2,
                    _ => unreachable!(),
                };

                rgb[i] = cubes;
            }

            if rgb[0] > R_CUBES || rgb[1] > G_CUBES || rgb[2] > B_CUBES {
                game_is_possible = false;
                break;
            }
        }

        if game_is_possible {
            sum += id;
        }
    }

    sum
}

fn part_one_2_0() -> u32 {
    let data = include_bytes!("../inputs/input.txt");
    data.split(|b| *b == b'\n')
        .enumerate()
        .filter_map(|(game_id, line)| {
            if line.is_empty() {
                return None;
            }
            let mut rgb = [0, 0, 0];
            line.splitn(2, |b| *b == b':')
                .nth(1)
                .unwrap()
                .split(|b| *b == b',' || *b == b';')
                .all(|item| {
                    let i = match item[1..].splitn(2, |b| *b == b' ').nth(1).unwrap() {
                        b"red" => 0usize,
                        b"green" => 1,
                        b"blue" => 2,
                        _ => unreachable!(),
                    };
                    rgb[i] = rgb[i].max(atoi::atoi(&item[1..]).unwrap());
                    rgb[i] <= 12 + i as u32
                })
                .then_some(game_id + 1)
        })
        .sum::<usize>() as u32
}

fn part_two(input: &str) -> u32 {
    let mut sum = 0;
    for game in input.lines() {
        let chunk: Vec<&str> = game.split(':').collect();

        let mut r: u32 = 0;
        let mut g: u32 = 0;
        let mut b: u32 = 0;

        for pull in chunk[1].split(';') {
            for color in pull.split(',') {
                let color_parts: Vec<&str> = color.trim().split(' ').collect();
                let cubes = color_parts[0].parse::<u32>().unwrap();

                let c: &mut u32 = match color_parts[1] {
                    "red" => &mut r,
                    "blue" => &mut b,
                    "green" => &mut g,
                    _ => unreachable!(),
                };

                if cubes > *c {
                    *c = cubes;
                }
            }
        }

        sum += r * b * g;
    }

    sum
}
