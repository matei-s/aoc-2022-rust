use std::{
    cmp::{max, min},
    io::stdin,
};

use cell::Cell;
use grid::Grid;

pub mod cell;
pub mod grid;

type Trace = Vec<(usize, usize)>;
type Cave = Grid<u8>;

const BG: &str = "\x1b[90m░░\x1b[0m";
const ROCK: &str = "\x1b[37m██\x1b[0m";
const SAND: &str = "\x1b[33m▒▒\x1b[0m";

fn main() {
    let lines: Vec<String> = stdin().lines().filter_map(Result::ok).collect();
    let (traces, max_y) = parse_traces(&lines);

    let (mut cave, spawn_cell) = make_cave(&traces, max_y);

    let (sand_to_floor, total_sand) = fill_with_sand(&mut cave, spawn_cell);

    println!("part 1: {sand_to_floor}");
    println!("part 2: {total_sand}");
}

fn fill_with_sand(cave: &mut Cave, spawn_cell: Cell) -> (u32, u32) {
    let mut total_sand = 0;

    'outer: loop {
        let mut sand = spawn_cell;
        loop {
            if sand.i as usize >= cave.h - 2 {
                break 'outer;
            } else if cave[sand.down()] == 0 {
                sand = sand.down();
            } else if cave[sand.dl()] == 0 {
                sand = sand.dl();
            } else if cave[sand.dr()] == 0 {
                sand = sand.dr();
            } else {
                cave[sand] = 2;
                total_sand += 1;
                break;
            }
        }
    }

    let sand_to_floor = total_sand;

    print_cave(&cave);

    loop {
        let mut sand = spawn_cell;
        if cave[sand] == 2 {
            break;
        }
        loop {
            if cave[sand.down()] == 0 {
                sand = sand.down();
            } else if cave[sand.dl()] == 0 {
                sand = sand.dl();
            } else if cave[sand.dr()] == 0 {
                sand = sand.dr();
            } else {
                cave[sand] = 2;
                total_sand += 1;
                break;
            }
        }
    }

    println!();
    print_cave(&cave);

    (sand_to_floor, total_sand)
}

fn parse_traces(lines: &Vec<String>) -> (Vec<Vec<(usize, usize)>>, usize) {
    let mut traces: Vec<Vec<(usize, usize)>> = Vec::new();

    let mut max_y = usize::MIN;

    for line in lines {
        let mut trace: Vec<(usize, usize)> = Vec::new();
        for xy in line.split(" -> ") {
            let tokens: Vec<&str> = xy.split(",").collect();
            let x = tokens[0].parse().unwrap();
            let y = tokens[1].parse().unwrap();
            max_y = max_y.max(y);
            trace.push((x, y));
        }
        traces.push(trace);
    }

    (traces, max_y)
}

fn make_cave(traces: &Vec<Trace>, max_y: usize) -> (Cave, Cell) {
    let mut cave = Grid::<u8>::new(2 * max_y + 5, max_y + 3);

    for trace in traces {
        for slice in trace.windows(2) {
            let (x1, y1) = slice[0];
            let (x2, y2) = slice[1];

            for i in min(y1, y2)..=max(y1, y2) {
                for j in min(x1, x2)..=max(x1, x2) {
                    let j = j + cave.w / 2 - 500;
                    cave[(i, j)] = 1;
                }
            }
        }
    }

    let floor_i = cave.h - 1;
    for j in 0..cave.w {
        cave[(floor_i, j)] = 1;
    }

    let spawn_cell = Cell {
        i: 0,
        j: cave.w as isize / 2,
    };

    cave[spawn_cell] = 3;

    (cave, spawn_cell)
}

fn print_cave(cave: &Cave) {
    for i in 0..cave.h {
        for j in 0..cave.w {
            match cave[(i, j)] {
                0 => print!("{BG}"),
                1 => print!("{ROCK}"),
                2 | 3 => print!("{SAND}"),
                _ => panic!("unrecognized cave cell"),
            }
        }
        println!();
    }
}
