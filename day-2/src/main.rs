use std::{cmp::Ordering, fs, io::BufRead};

#[derive(Debug)]
struct Match {
    round: usize,
    player_1: Weapon,
    player_2: Weapon,
    outcome: OutCome,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Clone, Copy)]
enum Weapon {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
enum OutCome {
    Draw,
    Win,
    Lose,
}

impl Ord for Weapon {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Weapon::Rock => match other {
                Weapon::Rock => Ordering::Equal,
                Weapon::Paper => Ordering::Less,
                Weapon::Scissors => Ordering::Greater,
            },
            Weapon::Paper => match other {
                Weapon::Rock => Ordering::Greater,
                Weapon::Paper => Ordering::Equal,
                Weapon::Scissors => Ordering::Less,
            },
            Weapon::Scissors => match other {
                Weapon::Rock => Ordering::Less,
                Weapon::Paper => Ordering::Greater,
                Weapon::Scissors => Ordering::Equal,
            },
        }
    }
}

fn get_weapon(s: &str) -> Weapon {
    match s {
        "A" | "X" => Weapon::Rock,
        "B" | "Y" => Weapon::Paper,
        _ => Weapon::Scissors,
    }
}

fn get_point(m: &Match) -> isize {
    let weapon_points = match m.player_2 {
        Weapon::Rock => 1,
        Weapon::Paper => 2,
        Weapon::Scissors => 3,
    };

    let outcome_points = match m.player_1.cmp(&m.player_2) {
        Ordering::Less => 6,
        Ordering::Equal => 3,
        Ordering::Greater => 0,
    };

    weapon_points + outcome_points
}

fn read_input() -> Vec<Match> {
    let contents = fs::read("input").expect("Not able to read the input");
    let mut count = 1;
    let mut matches: Vec<Match> = vec![];

    for line in contents.lines() {
        let line = line.unwrap();
        let (opp, pl) = line.split_once(" ").expect("Not able to split line");

        matches.push(Match {
            round: count,
            player_1: get_weapon(opp),
            player_2: get_weapon(pl),
            outcome: match pl {
                "X" => OutCome::Lose,
                "Y" => OutCome::Draw,
                _ => OutCome::Win,
            },
        });

        count += 1;
    }

    matches
}

fn part_1() {
    let matches = read_input();
    let result = matches.iter().fold(0, |acc, m| acc + get_point(m));

    println!("[Part 1] Result: {}", result);
}

fn part_2() {
    let result = read_input()
        .iter()
        .map(|m| {
            let p2_weapon = match m.outcome {
                OutCome::Draw => m.player_1,
                OutCome::Win => match m.player_1 {
                    Weapon::Rock => Weapon::Paper,
                    Weapon::Paper => Weapon::Scissors,
                    Weapon::Scissors => Weapon::Rock,
                },
                OutCome::Lose => match m.player_1 {
                    Weapon::Rock => Weapon::Scissors,
                    Weapon::Paper => Weapon::Rock,
                    Weapon::Scissors => Weapon::Paper,
                },
            };

            Match {
                round: m.round,
                player_1: m.player_1,
                player_2: p2_weapon,
                outcome: m.outcome,
            }
        })
        .fold(0, |acc, m| acc + get_point(&m));
    println!("[Part 2] Result: {}", result);
}

fn main() {
    part_1();
    part_2();
}
