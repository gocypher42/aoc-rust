const R_CUBES: u32 = 12;
const G_CUBES: u32 = 13;
const B_CUBES: u32 = 14;

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
}

fn part_one(input: &str) -> u32 {
    let mut sum = 0;
    for game in input.lines() {
        // println!("{}", game);
        let id_and_pulls: Vec<&str> = game.split(':').collect();

        let id: u32 = id_and_pulls[0].split(' ').collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();
        // println!("{}", id);

        let mut game_is_possible = true;

        for pull in id_and_pulls[1].split(';') {
            // println!("\t{}", pull);
            let mut r: u32 = 0;
            let mut g: u32 = 0;
            let mut b: u32 = 0;

            for color in pull.split(',') {
                let color_parts: Vec<&str> = color.trim().split(' ').collect();
                let cubes = color_parts[0].parse::<u32>().unwrap();

                let c: &mut u32 = match color_parts[1] {
                    "red" => &mut r,
                    "blue" => &mut b,
                    "green" => &mut g,
                    _ => unreachable!(),
                };

                *c = cubes;
            }

            if r > R_CUBES || b > B_CUBES || g > G_CUBES {
                // Game not possible
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
