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
    pub fn up(&self) -> Cell {
        *self + (-1, 0)
    }
    pub fn down(&self) -> Cell {
        *self + (1, 0)
    }
    pub fn left(&self) -> Cell {
        *self + (0, -1)
    }
    pub fn right(&self) -> Cell {
        *self + (0, 1)
    }
    pub fn dl(&self) -> Cell {
        *self + (1, -1)
    }
    pub fn dr(&self) -> Cell {
        *self + (1, 1)
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
