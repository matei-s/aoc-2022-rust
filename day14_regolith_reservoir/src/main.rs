use std::{
    cmp::{max, min},
    io::stdin,
    thread::sleep,
    time::Duration,
};

use cell::Cell;
use grid::Grid;

pub mod cell;
pub mod grid;

type Trace = Vec<(usize, usize)>;
type Cave = Grid<u8>;

fn main() {
    let lines: Vec<String> = stdin().lines().filter_map(Result::ok).collect();
    let (traces, min_x, max_x, max_y) = parse_traces(&lines);

    let (mut cave, spawn_cell) = make_cave(&traces, min_x, max_x, max_y);

    let total_sand = fill_with_sand(&mut cave, spawn_cell, false);

    println!("part 1: {total_sand}");
}

fn fill_with_sand(cave: &mut Cave, spawn_cell: Cell, animate: bool) -> u32 {
    'outer: loop {
        let mut sand = spawn_cell;

        loop {
            if animate {
                clearscreen::clear().expect("Failed to clear the screen");
                print_cave(&cave, sand);
                sleep(Duration::from_millis(20));
            }

            if sand.i as usize >= cave.h - 1 {
                break 'outer;
            } else if cave[sand.down()] == 0 {
                sand = sand.down();
            } else if cave[sand.dl()] == 0 {
                sand = sand.dl();
            } else if cave[sand.dr()] == 0 {
                sand = sand.dr();
            } else {
                cave[sand] = 2;
                break;
            }
        }
    }

    let mut total_sand = 0;
    for i in 0..cave.h {
        for j in 0..cave.w {
            if cave[(i, j)] == 2 {
                total_sand += 1
            }
        }
    }

    if !animate {
        print_cave(cave, spawn_cell);
    }

    total_sand
}

fn parse_traces(lines: &Vec<String>) -> (Vec<Vec<(usize, usize)>>, usize, usize, usize) {
    let mut traces: Vec<Vec<(usize, usize)>> = Vec::new();

    let mut min_x = usize::MAX;
    let mut max_x = usize::MIN;
    let mut max_y = usize::MIN;

    for line in lines {
        let mut trace: Vec<(usize, usize)> = Vec::new();
        for xy in line.split(" -> ") {
            let tokens: Vec<&str> = xy.split(",").collect();
            let x = tokens[0].parse().unwrap();
            let y = tokens[1].parse().unwrap();
            min_x = min_x.min(x);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
            trace.push((x, y));
        }
        traces.push(trace);
    }

    (traces, min_x, max_x, max_y)
}

fn make_cave(traces: &Vec<Trace>, min_x: usize, max_x: usize, max_y: usize) -> (Cave, Cell) {
    let mut cave = Grid::<u8>::new(max_x - min_x + 3, max_y + 2);

    for trace in traces {
        for slice in trace.windows(2) {
            let (x1, y1) = slice[0];
            let (x2, y2) = slice[1];

            for i in min(y1, y2)..=max(y1, y2) {
                for j in min(x1, x2)..=max(x1, x2) {
                    let j = j - min_x + 1;
                    cave[(i, j)] = 1;
                }
            }
        }
    }

    let spawn_cell = Cell {
        i: 0,
        j: 500 - min_x as isize + 1,
    };

    (cave, spawn_cell)
}

fn print_cave(cave: &Cave, sand_cell: Cell) {
    for i in 0..cave.h {
        for j in 0..cave.w {
            if (Cell {
                i: i as isize,
                j: j as isize,
            }) == sand_cell
            {
                print!("\x1b[33m▓▓\x1b[0m");
                continue;
            }
            match cave[(i, j)] {
                0 => print!("\x1b[90m░░\x1b[0m"),
                1 => print!("\x1b[37m██\x1b[0m"),
                2 | 3 => print!("\x1b[33m▒▒\x1b[0m"),
                _ => panic!("unrecognized cave cell"),
            }
        }
        println!();
    }
}
