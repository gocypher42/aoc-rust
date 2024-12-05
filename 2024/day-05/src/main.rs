use std::collections::HashMap;

#[allow(clippy::wildcard_imports)]
use utils::*;

fn main() {
    aoc_main!("../inputs/input.txt");
}

fn parse_input(input: &str) -> (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut pages = vec![];
    let mut in_rules = true;

    input.lines().for_each(|l| {
        if l.is_empty() {
            in_rules = false;
            return;
        }

        if in_rules {
            let (before, after) = l.split_once('|').unwrap();
            let before: usize = before.parse().unwrap();
            let after: usize = after.parse().unwrap();
            rules.entry(before).or_default().push(after);
            return;
        }

        pages.push(l.split(',').map(|x| x.parse().unwrap()).collect());
    });

    (rules, pages)
}

fn part_one(input: &str) -> usize {
    let (rules, pages_list) = parse_input(input);

    pages_list
        .into_iter()
        .filter_map(|pages| {
            for (i, page) in pages.iter().enumerate().rev() {
                if let Some(list) = rules.get(page) {
                    if pages[..i].iter().any(|p| list.contains(p)) {
                        return None;
                    }
                }
            }
            Some(*pages.get(pages.len() / 2).unwrap())
        })
        .sum()
}

fn part_two(input: &str) -> usize {
    let (rules, pages_list) = parse_input(input);

    pages_list
        .into_iter()
        .filter_map(|mut pages| {
            if order_pages(&rules, &mut pages) {
                return None;
            }

            Some(pages[pages.len() / 2])
        })
        .sum()
}

fn order_pages(rules: &HashMap<usize, Vec<usize>>, pages: &mut Vec<usize>) -> bool {
    let mut pages_to_replace: Vec<usize> = vec![];
    for (i, page) in pages.iter().enumerate().rev() {
        if let Some(list) = rules.get(page) {
            for p in &pages[..i] {
                if list.contains(p) && !pages_to_replace.contains(page) {
                    pages_to_replace.push(*page);
                }
            }
        }
    }

    if pages_to_replace.is_empty() {
        return true;
    }

    pages.retain(|x| !pages_to_replace.contains(x));

    for page_to_place in pages_to_replace {
        let page_rules = rules.get(&page_to_place).unwrap();
        for i in (0..pages.len()).rev() {
            if !pages[..i].iter().any(|x| page_rules.contains(x)) {
                pages.insert(i, page_to_place);
                break;
            }
        }
    }
    false
}
