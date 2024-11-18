const R_CUBES: u32 = 12;
const G_CUBES: u32 = 13;
const B_CUBES: u32 = 14;
const T_CUBES: u32 = R_CUBES + G_CUBES + B_CUBES;

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
    println!("Part 2 ({part_one_time}): {part_two_result:?}");
    println!("------");
}

fn part_one(input: &str) -> u32 {
    let mut sum = 0;
    for game in input.lines() {
        println!("{}", game);
        let chunk: Vec<&str> = game.split(':').collect();

        let id: u32 = chunk[0].split(' ').collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();
        println!("{}", id);

        let mut game_is_possible = true;

        for pull in chunk[1].split(';') {
            println!("\t{}", pull);

            let mut r: u32 = 0;
            let mut g: u32 = 0;
            let mut b: u32 = 0;

            for color in pull.split(',') {
                let color_parts: Vec<&str> = color.trim().split(' ').collect();
                let cubes = color_parts[0].parse::<u32>().unwrap();

                match color_parts[1] {
                    "red" => r = cubes,
                    "blue" => b = cubes,
                    "green" => g = cubes,
                    _ => unreachable!(),
                }
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
    0
}
