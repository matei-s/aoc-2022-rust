use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().filter_map(Result::ok).collect();

    part_1(&lines);
}

fn part_1(lines: &Vec<String>) {
    let mut priorities = 0;
    for line in lines {
        let comp1: HashSet<char> = line[..line.len() / 2].chars().into_iter().collect();
        let comp2: HashSet<char> = line[line.len() / 2..].chars().into_iter().collect();

        for c in comp1.intersection(&comp2) {
            let priority = priority_of(*c as u8);

            println!("{:?} {priority}", comp1.intersection(&comp2));
            priorities += priority;
        }
    }

    println!("part 1: {priorities}")
}

fn priority_of(c: u8) -> u32 {
    if c <= b'z' && c >= b'a' {
        (1 + c - b'a') as u32
    } else {
        (27 + c - b'A') as u32
    }
}
