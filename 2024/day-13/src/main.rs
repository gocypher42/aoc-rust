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
        let mut m: [[f64; 3]; 2] = [
            [self.a.x as f64, self.b.x as f64, self.target.x as f64],
            [self.a.y as f64, self.b.y as f64, self.target.y as f64],
        ];

        m[0][1] = m[0][1] / m[0][0];
        m[0][2] = m[0][2] / m[0][0];
        m[0][0] = m[0][0] / m[0][0];

        m[1][1] = m[1][1] - (m[0][1] * m[1][0]);
        m[1][2] = m[1][2] - (m[0][2] * m[1][0]);
        m[1][0] = m[1][0] - (m[0][0] * m[1][0]);

        if m[1][1] < 0.0 && m[1][2] < 0.0 {
            m[1][1] *= -1.0;
            m[1][2] *= -1.0;
        }

        let b = m[1][2] / m[1][1];
        let a = m[0][2] - (m[0][1] * b);

        if b < 0.0 || a < 0.0 {
            return None;
        }

        let dx = (a.round() * (self.a.x as f64) + b.round() * (self.b.x as f64)) as usize;
        let dy = (a.round() * (self.a.y as f64) + b.round() * (self.b.y as f64)) as usize;
        if self.target.x != dx || self.target.y != dy {
            return None;
        }

        Some((a, b))
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
