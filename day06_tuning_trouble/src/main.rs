use std::collections::HashSet;
use std::io::{stdin, BufRead};

fn main() {
    let lines: Vec<String> = stdin().lock().lines().filter_map(Result::ok).collect();

    println!("part 1:");
    for line in lines.iter() {
        println!("{}", find_marker(&line, 4));
    }

    println!("part 1:");
    for line in lines.iter() {
        println!("{}", find_marker(&line, 14));
    }
}

fn find_marker(seq: &String, length: usize) -> usize {
    for (i, marker) in seq.as_bytes().windows(length).enumerate() {
        let set: HashSet<u8> = marker.iter().cloned().collect();
        if set.len() == length {
            return i + length;
        }
    }

    panic!("no marker found");
}
