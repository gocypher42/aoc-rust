#[allow(clippy::wildcard_imports)]
use utils::*;

fn main() {
    aoc_main!("../inputs/input.txt");
}

#[derive(Default, Debug)]
struct Config {
    a: Position,
    b: Position,
    target: Position,
}

impl Config {
    fn resolve(&self) -> Option<(f64, f64)> {
        let m: [[f64; 2]; 2] = [
            [self.a.x as f64, self.b.x as f64],
            [self.a.y as f64, self.b.y as f64],
        ];

        let f = 1.0 / ((m[0][0] * m[1][1]) - (m[0][1] * m[1][0]));

        let m_i: [[f64; 2]; 2] = [
            [m[1][1] * f, m[0][1] * -1.0 * f],
            [m[1][0] * -1.0 * f, m[0][0] * f],
        ];

        let c_a = m_i[0][0] * (self.target.x as f64) + m_i[0][1] * (self.target.y as f64);
        let c_b = m_i[1][0] * (self.target.x as f64) + m_i[1][1] * (self.target.y as f64);

        if c_b < 0.0 || c_a < 0.0 {
            return None;
        }

        let dx = (c_a.round() * m[0][0] + c_b.round() * m[0][1]) as usize;
        let dy = (c_a.round() * m[1][0] + c_b.round() * m[1][1]) as usize;

        if self.target.x != dx || self.target.y != dy {
            return None;
        }

        Some((c_a, c_b))
    }
}

fn part_one(input: &str) -> usize {
    let mut configs: Vec<Config> = vec![];
    {
        let mut current_config = Config::default();
        for line in input.lines() {
            if line.is_empty() {
                configs.push(current_config);
                current_config = Config::default();
                continue;
            }
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts[0] == "Button" {
                let x: usize = parts[2][2..parts[2].len() - 1].parse().unwrap();
                let y: usize = parts[3][2..].parse().unwrap();

                if parts[1][..1] == *"A" {
                    current_config.a.x = x;
                    current_config.a.y = y;
                } else if parts[1][..1] == *"B" {
                    current_config.b.x = x;
                    current_config.b.y = y;
                }
            } else if parts[0] == "Prize:" {
                current_config.target.x = parts[1][2..parts[1].len() - 1].parse().unwrap();
                current_config.target.y = parts[2][2..].parse().unwrap();
            }
        }
        configs.push(current_config);
    }
    let configs = configs;

    configs
        .iter()
        .filter_map(|config| {
            config
                .resolve()
                .map(|(a, b)| (a * 3.0 + b * 1.0).round() as usize)
        })
        .sum()
}

fn part_two(input: &str) -> usize {
    let mut configs: Vec<Config> = vec![];
    {
        let mut current_config = Config::default();
        for line in input.lines() {
            if line.is_empty() {
                configs.push(current_config);
                current_config = Config::default();
                continue;
            }
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts[0] == "Button" {
                let x: usize = parts[2][2..parts[2].len() - 1].parse().unwrap();
                let y: usize = parts[3][2..].parse().unwrap();

                if parts[1][..1] == *"A" {
                    current_config.a.x = x;
                    current_config.a.y = y;
                } else if parts[1][..1] == *"B" {
                    current_config.b.x = x;
                    current_config.b.y = y;
                }
            } else if parts[0] == "Prize:" {
                current_config.target.x =
                    parts[1][2..parts[1].len() - 1].parse::<usize>().unwrap() + 10000000000000;
                current_config.target.y = parts[2][2..].parse::<usize>().unwrap() + 10000000000000;
            }
        }
        configs.push(current_config);
    }
    let configs = configs;

    configs
        .iter()
        .filter_map(|config| {
            config
                .resolve()
                .map(|(a, b)| (a * 3.0 + b * 1.0).round() as usize)
        })
        .sum()
}
