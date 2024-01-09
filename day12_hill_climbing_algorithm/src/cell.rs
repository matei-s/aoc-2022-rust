#![allow(dead_code)]

use std::ops::{Add, AddAssign};

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub struct Cell {
    pub i: isize,
    pub j: isize,
}

impl Cell {
    pub fn new() -> Cell {
        Cell { i: 0, j: 0 }
    }
}

impl Add for Cell {
    type Output = Cell;

    fn add(self, other: Cell) -> Cell {
        Cell {
            i: self.i + other.i,
            j: self.j + other.j,
        }
    }
}

impl Add<(isize, isize)> for Cell {
    type Output = Cell;

    fn add(self, (i, j): (isize, isize)) -> Cell {
        Cell {
            i: self.i + i,
            j: self.j + j,
        }
    }
}

impl AddAssign for Cell {
    fn add_assign(&mut self, other: Cell) {
        *self = Cell {
            i: self.i + other.i,
            j: self.j + other.j,
        };
    }
}
