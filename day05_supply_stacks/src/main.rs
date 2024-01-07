use std::io::{stdin, BufRead};

fn main() {
    let lines: Vec<String> = stdin().lock().lines().filter_map(Result::ok).collect();

    let stack_lines: Vec<String> = lines
        .iter()
        .take_while(|line| line.contains("["))
        .cloned()
        .collect();

    let stacks = parse_stacks(&stack_lines);
    let moves = parse_moves(&lines[stack_lines.len() + 2..]);

    print_stacks(&stacks);

    println!("part 1: {}", part_1(&stacks, &moves));
    println!("part 2: {}", part_2(&stacks, &moves));
}

fn part_1(stacks: &Vec<Vec<char>>, moves: &Vec<(usize, usize, usize)>) -> String {
    let mut stacks = stacks.clone();

    for &(count, from, to) in moves {
        for _ in 0..count {
            let el = stacks[from].pop().expect("invalid stack");
            stacks[to].push(el);
        }
    }

    read_stacks(&stacks)
}

fn part_2(stacks: &Vec<Vec<char>>, moves: &Vec<(usize, usize, usize)>) -> String {
    let mut stacks = stacks.clone();

    for &(count, from, to) in moves {
        let at = stacks[from].len() - count;
        let mut tail = stacks[from].split_off(at);
        stacks[to].append(&mut tail);
    }

    read_stacks(&stacks)
}

fn parse_moves(move_lines: &[String]) -> Vec<(usize, usize, usize)> {
    let moves: Vec<(usize, usize, usize)> = move_lines
        .iter()
        .map(|line| {
            let move_info: Vec<usize> = line
                .replace("move", "")
                .replace("from", "")
                .replace("to", "")
                .split_whitespace()
                .map(|nstr| nstr.parse::<usize>())
                .filter_map(Result::ok)
                .collect();

            (move_info[0], move_info[1] - 1, move_info[2] - 1)
        })
        .collect();
    moves
}

fn parse_stacks(lines: &Vec<String>) -> Vec<Vec<char>> {
    let width = (lines.last().expect("invalid line").len() + 1) / 4;
    let mut stacks: Vec<Vec<char>> = vec![vec![]; width];

    for line in lines.iter().rev() {
        for i in 0..width {
            let char = line.as_bytes()[i * 4 + 1];
            if char != b' ' {
                stacks[i].push(char as char);
            }
        }
    }

    stacks
}

fn read_stacks(stacks: &Vec<Vec<char>>) -> String {
    let mut s = String::new();

    for stack in stacks {
        let el = stack.last().expect("invalid stack");
        s.push(*el);
    }

    s
}

fn print_stacks(stacks: &Vec<Vec<char>>) {
    for stack in stacks {
        for c in stack {
            print!("{c} ");
        }
        println!();
    }
}
