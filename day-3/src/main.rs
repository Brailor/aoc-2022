use std::collections::{btree_set::Intersection, BTreeSet};
fn read_input() -> std::str::Lines<'static> {
    include_str!("../input").lines()
}

fn part_1() {
    let res: u32 = read_input()
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);
            let first: BTreeSet<char> = first.chars().collect();
            let second: BTreeSet<char> = second.chars().collect();
            let res: Vec<char> = first.intersection(&second).cloned().collect();
            let res = res.first().unwrap();
            let res = if res.is_lowercase() {
                res.to_owned() as u32 - 96
            } else {
                res.to_owned() as u32 - 38
            };

            res
        })
        .sum();

    println!("[Part 1] Result: {res}");
}

fn part_2() {
    let input: Vec<BTreeSet<char>> = read_input().map(|line| line.chars().collect()).collect();
    let res: u32 = input
        .chunks(3)
        .flat_map(|chunk| {
            let c1 = chunk[0].clone();
            let c2 = chunk[1].clone();
            let c3 = chunk[2].clone();
            let res: BTreeSet<char> = c1.intersection(&c2).cloned().collect();
            let res: Vec<char> = res.intersection(&c3).cloned().collect();
            res
        })
        .map(|res| {
            let res = if res.is_lowercase() {
                res.to_owned() as u32 - 96
            } else {
                res.to_owned() as u32 - 38
            };

            res
        })
        .sum();
    println!("[Part 2] Result: {res}");
}

fn main() {
    part_1();
    part_2();
}
