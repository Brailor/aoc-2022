use std::{cmp::Ordering, fs, io::BufRead};

#[derive(Debug)]
struct Elf {
    calories_carried: usize,
}

fn read_input() -> Vec<Elf> {
    let contents = fs::read("input").expect("Not able to read input file");
    let mut elf = Elf {
        calories_carried: 0,
    };
    let mut elfs: Vec<Elf> = vec![];

    for line in contents.lines() {
        if line.is_ok() {
            let line = line.unwrap();
            if line.is_empty() {
                elfs.push(elf);

                elf = Elf {
                    calories_carried: 0,
                };
                continue;
            }

            let num: usize = line.parse().unwrap();
            elf.calories_carried += num;
        }
    }

    elfs
}

fn part_1() {
    let elfs = read_input();
    let biggest = elfs.iter().max_by(|x, y| {
        if x.calories_carried > y.calories_carried {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    println!("Elf with the most calories: {:?}", biggest.unwrap());
}

fn part_2() {
    let mut elfs = read_input();

    elfs.sort_by(|x, y| {
        if x.calories_carried > y.calories_carried {
            Ordering::Greater
        } else if x.calories_carried == y.calories_carried {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    });

    let biggest_three = elfs.iter().rev().take(3).fold(0, |mut acc, elf| {
        acc += elf.calories_carried;
        acc
    });

    println!("Total of the largest 3 carriers: {}", biggest_three);
}

fn main() {
    part_1();

    part_2();
}
