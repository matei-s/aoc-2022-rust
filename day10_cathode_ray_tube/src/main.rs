use std::{collections::HashMap, io::stdin};

fn main() {
    let instructions: Vec<String> = stdin().lines().filter_map(Result::ok).collect();

    let signal_strenghts = compute_signal_strenghts(&instructions);
    println!("part 1: {:?}", signal_strenghts);

    let screen = render_screen(&instructions);
    println!("part 2:");
    print_screen(&screen);
}

fn compute_signal_strenghts(instructions: &Vec<String>) -> i32 {
    let mut signal_strenghts = HashMap::<u32, i32>::new();

    let mut reg_x = 1i32;
    let mut cycles = 0u32;

    let checkpoints = [20, 60, 100, 140, 180, 220];
    let mut c_i = 0;

    for instr in instructions {
        let tokens: Vec<&str> = instr.split_whitespace().collect();
        let mut added_value = 0;
        match tokens.as_slice() {
            ["noop"] => {
                cycles += 1;
            }
            ["addx", val] => {
                let val: i32 = val.parse().unwrap();
                cycles += 2;
                added_value = val;
            }
            _ => {
                panic!("unrecognized command: {:?}", tokens);
            }
        }

        if c_i < checkpoints.len() && cycles >= checkpoints[c_i] {
            signal_strenghts.insert(checkpoints[c_i], reg_x);
            c_i += 1;
        }
        reg_x += added_value;
    }

    signal_strenghts
        .iter()
        .fold(0, |acc, (k, v)| acc + *k as i32 * v)
}

fn render_screen(instructions: &Vec<String>) -> Vec<bool> {
    let mut reg_x: isize = 1;
    let mut cycle: isize = 0;

    let mut screen = vec![false; 40 * 6];

    render_pixel(&mut screen, reg_x, cycle);

    for instr in instructions {
        let tokens: Vec<&str> = instr.split_whitespace().collect();
        match tokens.as_slice() {
            ["noop"] => {
                cycle += 1;
            }
            ["addx", val] => {
                let val: isize = val.parse().unwrap();
                render_pixel(&mut screen, reg_x, cycle + 1);
                cycle += 2;
                reg_x += val;
            }
            _ => {
                panic!("unrecognized command: {:?}", tokens);
            }
        }
        render_pixel(&mut screen, reg_x, cycle);
    }

    screen
}

fn render_pixel(screen: &mut Vec<bool>, reg_x: isize, cycle: isize) {
    let col = cycle % 40;
    if (reg_x - col).abs() <= 1 {
        screen[cycle as usize] = true;
    }
}

fn print_screen(screen: &Vec<bool>) {
    for i in 0..6 {
        for j in 0..40 {
            match screen[i * 40 + j] {
                false => print!("  "),
                true => print!("██"),
            }
        }
        println!();
    }
}
