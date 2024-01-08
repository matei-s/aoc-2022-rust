use std::{collections::HashSet, io::stdin};

struct Rope {
    h: (i32, i32),
    t: (i32, i32),
}

enum Direction {
    U,
    D,
    L,
    R,
}

impl Rope {
    fn new() -> Self {
        Rope {
            h: (0, 0),
            t: (0, 0),
        }
    }
    fn step(&mut self, dir: &Direction) {
        let (hi, hj) = self.h;
        self.h = match dir {
            Direction::U => (hi - 1, hj),
            Direction::D => (hi + 1, hj),
            Direction::L => (hi, hj - 1),
            Direction::R => (hi, hj + 1),
        };

        let (hi, hj) = self.h;
        let (ti, tj) = self.t;

        self.t = match (hi - ti, hj - tj) {
            (2, _) => (hi - 1, hj),
            (-2, _) => (hi + 1, hj),
            (_, 2) => (hi, hj - 1),
            (_, -2) => (hi, hj + 1),
            _ => match ((hi - ti).abs(), (hj - tj).abs()) {
                (1, 1) | (0, 1) | (1, 0) | (0, 0) => (ti, tj),
                _ => panic!("unexpected positions h:{:?} t:{:?}", self.h, self.t),
            },
        }
    }
}

impl Direction {
    fn parse_move(s: &str) -> (Direction, u32) {
        let tokens: Vec<&str> = s.split_whitespace().collect();
        let dir = tokens[0];
        let steps = tokens[1].parse().unwrap();
        match dir {
            "U" => (Direction::U, steps),
            "R" => (Direction::R, steps),
            "D" => (Direction::D, steps),
            "L" => (Direction::L, steps),
            _ => {
                panic!("unrecognized move: {:?}", tokens);
            }
        }
    }
}

fn main() {
    let lines: Vec<String> = stdin().lines().filter_map(Result::ok).collect();

    let mut rope = Rope::new();

    let mut tail_positions = HashSet::<(i32, i32)>::new();

    tail_positions.insert(rope.t);

    for line in lines {
        let (dir, steps) = Direction::parse_move(&line);
        for _ in 0..steps {
            rope.step(&dir);
            tail_positions.insert(rope.t);
        }
    }

    println!("part 1: {}", tail_positions.len());
}
