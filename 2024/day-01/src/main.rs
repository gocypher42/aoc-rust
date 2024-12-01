use utils::*;

fn main() {
    aoc_main!("../inputs/input.txt");
}

fn part_one(input: &str) -> usize {
    let mut l1 = vec![];
    let mut l2 = vec![];
    for line in input.lines() {
        // println!("{:?}", line);
        let (n1, n2) = line.split_once("   ").unwrap();
        let (n1, n2): (u32, u32) = (n1.parse().unwrap(), n2.parse().unwrap());
        l1.push(n1);
        l2.push(n2);
        println!("{n1:?} {n2:?}");
    }
    l1.sort();
    l2.sort();

    print_2d_slice(&l1);
    print_2d_slice(&l2);

    let l1 = l1;
    let l2 = l2;

    let mut sum: usize = 0;

    for i in 0..l1.len() {
        if l1[i] > l2[i] {
            sum += (l1[i] - l2[i]) as usize;
        } else {
            sum += (l2[i] - l1[i]) as usize;
        }
    }

    sum
}

fn part_two(input: &str) -> usize {
    let mut l1 = vec![];
    let mut l2 = vec![];
    for line in input.lines() {
        // println!("{:?}", line);
        let (n1, n2) = line.split_once("   ").unwrap();
        let (n1, n2): (u32, u32) = (n1.parse().unwrap(), n2.parse().unwrap());
        l1.push(n1);
        l2.push(n2);
        println!("{n1:?} {n2:?}");
    }
    l1.sort();
    l2.sort();

    // print_2d_slice(&l1);
    // print_2d_slice(&l2);
    println!();

    let l1 = l1;
    let l2 = l2;

    l1.iter()
        .map(|i| (*i as usize) * l2.iter().map(|x| if x == i { 1 } else { 0 }).sum::<usize>())
        .sum()
}
