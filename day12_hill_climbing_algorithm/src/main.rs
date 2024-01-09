pub mod cell;
pub mod grid;

use cell::Cell;
use grid::Grid;
use std::{collections::LinkedList, io::stdin};

fn main() {
    let lines: Vec<String> = stdin().lines().filter_map(Result::ok).collect();

    let mut hm = Grid::parse(&lines);

    let start = hm.find(b'S').unwrap();
    let end = hm.find(b'E').unwrap();

    hm[start] = b'a';
    hm[end] = b'z';

    let path_length = find_path(&hm, start, end);

    println!("part 1: {path_length}");
}

fn find_path(hm: &Grid<u8>, start: Cell, end: Cell) -> u32 {
    let mut queue = LinkedList::<Cell>::new();
    let mut distances = Grid::<u32>::new(hm.w, hm.h);
    distances.init(u32::MAX);

    queue.push_back(start);
    distances[start] = 0;

    while !queue.is_empty() {
        let cell = queue.pop_front().unwrap();
        let dist = distances[cell];

        let neighbors = get_neighbors(hm, cell);

        for neighbor in neighbors.iter() {
            if dist + 1 < distances[*neighbor] {
                distances[*neighbor] = dist + 1;
                queue.push_back(*neighbor);
            }
        }
    }

    distances[end]
}

fn get_neighbors(hm: &Grid<u8>, cell: Cell) -> Vec<Cell> {
    let mut neighbors = Vec::new();

    for step in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let n_cell = cell + step;

        if !(0..hm.h).contains(&(n_cell.i as usize)) || !(0..hm.w).contains(&(n_cell.j as usize)) {
            continue;
        }

        if hm[n_cell] > hm[cell] + 1 {
            continue;
        }

        neighbors.push(n_cell);
    }

    neighbors
}
