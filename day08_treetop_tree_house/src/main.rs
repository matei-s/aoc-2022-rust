use std::{
    fmt::{Debug, Display},
    io::stdin,
    ops::{Index, IndexMut},
};

struct Grid<T> {
    v: Vec<T>,
    w: usize,
    h: usize,
}

impl Grid<u8> {
    fn parse(lines: &Vec<String>) -> Grid<u8> {
        let mut v: Vec<u8> = Vec::new();
        let mut w = 0usize;
        let mut h = 0usize;

        for line in lines {
            let mut line = line.as_bytes().to_vec();
            w = line.len();
            h += 1;

            v.append(&mut line);
        }

        for i in 0..v.len() {
            v[i] -= b'0';
        }

        Grid { v, w, h }
    }
}

impl<T: Default + Clone + Display + Debug> Grid<T> {
    fn new(w: usize, h: usize) -> Grid<T> {
        Grid {
            v: vec![T::default(); w * h],
            w,
            h,
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        for i in 0..self.h {
            for j in 0..self.w {
                print!("{:?} ", self[(i, j)]);
            }
            println!();
        }
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (i, j): (usize, usize)) -> &T {
        &self.v[i * self.w + j]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut T {
        &mut self.v[i * self.w + j]
    }
}

fn main() {
    let lines: Vec<String> = stdin().lines().filter_map(Result::ok).collect();

    let grid = Grid::parse(&lines);

    println!("part 1: {}", compute_visible(&grid));
    println!("part 2: {}", compute_scenic_score(&grid));
}

fn compute_scenic_score(grid: &Grid<u8>) -> usize {
    let mut up: Grid<usize> = Grid::new(grid.w, grid.h);
    let mut down: Grid<usize> = Grid::new(grid.w, grid.h);
    let mut left: Grid<usize> = Grid::new(grid.w, grid.h);
    let mut right: Grid<usize> = Grid::new(grid.w, grid.h);

    // viewing distance up-down
    for j in 0..grid.w {
        let mut last_stop = vec![0usize; 10];
        for i in 0..grid.h {
            let stop = last_stop[grid[(i, j)] as usize..].iter().max().unwrap();
            up[(i, j)] = *stop;
            last_stop[grid[(i, j)] as usize] = i;
        }

        last_stop = vec![grid.h - 1; 10];
        for i in (0..grid.h).rev() {
            let stop = last_stop[grid[(i, j)] as usize..].iter().min().unwrap();
            down[(i, j)] = *stop;
            last_stop[grid[(i, j)] as usize] = i;
        }
    }

    // viewing distance left-right
    for i in 0..grid.h {
        let mut last_stop = vec![0usize; 10];

        for j in 0..grid.w {
            let stop = last_stop[grid[(i, j)] as usize..].iter().max().unwrap();
            left[(i, j)] = *stop;
            last_stop[grid[(i, j)] as usize] = j;
        }

        last_stop = vec![grid.w - 1; 10];
        for j in (0..grid.w).rev() {
            let stop = last_stop[grid[(i, j)] as usize..].iter().min().unwrap();
            right[(i, j)] = *stop;
            last_stop[grid[(i, j)] as usize] = j;
        }
    }

    let mut max = 0usize;
    for i in 0..grid.h {
        for j in 0..grid.w {
            let scene_up = i - up[(i, j)];
            let scene_down = down[(i, j)] - i;
            let scene_left = j - left[(i, j)];
            let scene_right = right[(i, j)] - j;

            let scenic_score = scene_up * scene_down * scene_left * scene_right;

            if scenic_score > max {
                max = scenic_score;
            }
        }
    }

    max
}

fn compute_visible(grid: &Grid<u8>) -> usize {
    let w = grid.w;
    let h = grid.h;

    let mut visible: Grid<bool> = Grid::new(grid.w, grid.h);

    // edges
    for i in 0..grid.h {
        visible[(i, 0)] = true;
        visible[(i, w - 1)] = true;
    }
    for j in 0..w {
        visible[(0, j)] = true;
        visible[(h - 1, j)] = true;
    }

    // along rows
    for i in 1..h - 1 {
        let mut max = 0;

        for j in 0..w {
            if grid[(i, j)] > max {
                visible[(i, j)] = true;
                max = grid[(i, j)];
            }
        }
        max = 0;
        for j in (0..w).rev() {
            if grid[(i, j)] > max {
                visible[(i, j)] = true;
                max = grid[(i, j)];
            }
        }
    }

    // along columns

    for j in 1..w - 1 {
        let mut max = 0u8;

        for i in 0..h {
            if grid[(i, j)] > max {
                visible[(i, j)] = true;
                max = grid[(i, j)];
            }
        }

        max = 0;
        for i in (0..h).rev() {
            if grid[(i, j)] > max {
                visible[(i, j)] = true;
                max = grid[(i, j)];
            }
        }
    }

    visible.v.iter().filter(|b| **b).count()
}
