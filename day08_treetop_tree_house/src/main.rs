use std::io::stdin;

fn main() {
    let lines = stdin().lines().filter_map(Result::ok);

    let mut grid: Vec<u8> = Vec::new();

    let mut w = 0usize;
    let mut h = 0usize;

    for line in lines {
        let mut line = Vec::from(line);
        w = line.len();
        h += 1;

        grid.append(&mut line);
    }

    println!("part 1: {}", compute_visible(&grid, w, h));
}

fn compute_visible(grid: &Vec<u8>, w: usize, h: usize) -> usize {
    let mut visible = vec![false; grid.len()];

    // edges
    for i in 0..h {
        visible[i * w] = true;
        visible[i * w + w - 1] = true;
    }
    for j in 0..w {
        visible[j] = true;
        visible[(h - 1) * w + j] = true;
    }

    // along rows
    for i in 1..h - 1 {
        let mut max = 0;

        let range = i * w..i * w + w;

        for index in range.clone() {
            if grid[index] > max {
                visible[index] = true;
                max = grid[index];
            }
        }
        max = 0;
        for index in range.rev() {
            if grid[index] > max {
                visible[index] = true;
                max = grid[index];
            }
        }
    }

    // along columns

    for j in 1..w - 1 {
        let mut max = 0u8;

        let range = j..=(h - 1) * w + j;

        for index in range.clone().step_by(w) {
            if grid[index] > max {
                visible[index] = true;
                max = grid[index];
            }
        }

        max = 0;
        for index in range.rev().step_by(w) {
            if grid[index] > max {
                visible[index] = true;
                max = grid[index];
            }
        }
    }

    visible.iter().filter(|b| **b).count()
}

fn display_visible(visible: &Vec<bool>, w: usize, h: usize) {
    for i in 0..h {
        for j in 0..w {
            let symbol = if visible[i * w + j] { "*" } else { "." };
            print!("{symbol} ");
        }
        println!();
    }
}
