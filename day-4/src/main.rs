fn read_input() -> &'static str {
    include_str!("../input")
}

fn parse_inputs(args: [&str; 4]) -> [usize; 4] {
    args.map(|arg| arg.parse::<usize>().unwrap())
}

fn part_1() {
    let result: usize = read_input()
        .lines()
        .filter(|line| {
            let split = line.split_once(",");

            match split {
                Some(s) => {
                    let (first, second) = s;
                    let (f_b, f_e) = first.split_once("-").expect("Cant split by `-`");
                    let (s_b, s_e) = second.split_once("-").expect("Cant split by `-`");
                    let [f_b, f_e, s_b, s_e] = parse_inputs([f_b, f_e, s_b, s_e]);

                    if (f_b >= s_b) && (f_e <= s_e) {
                        true
                    } else if (s_b >= f_b) && (s_e <= f_e) {
                        true
                    } else {
                        false
                    }
                }
                None => false,
            }
        })
        .count();

    println!("[Part 1] The result is: {}", result)
}

fn part_2() {
    let result: usize = read_input()
        .lines()
        .filter(|line| {
            let split = line.split_once(",");

            match split {
                Some(s) => {
                    let (first, second) = s;
                    let (f_b, f_e) = first.split_once("-").expect("Cant split by `-`");
                    let (s_b, s_e) = second.split_once("-").expect("Cant split by `-`");
                    let [f_b, f_e, s_b, s_e] = parse_inputs([f_b, f_e, s_b, s_e]);

                    if (f_b >= s_b) && (f_e <= s_e) {
                        true
                    } else if (s_b >= f_b) && (s_e <= f_e) {
                        true
                    } else if f_e <= s_b && f_b >= s_e {
                        true
                    } else if f_e >= s_b && f_b <= s_e {
                        true
                    } else {
                        false
                    }
                }
                None => false,
            }
        })
        .count();

    println!("[Part 2] The result is: {}", result)
}

fn main() {
    part_1();
    part_2();
}
