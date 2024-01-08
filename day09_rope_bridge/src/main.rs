use std::{
    collections::HashSet,
    io::stdin,
    ops::{Add, AddAssign},
};

struct Rope {
    v: Vec<Coord>,
}

#[derive(Debug)]
enum Direction {
    U,
    D,
    L,
    R,
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
struct Coord {
    i: i32,
    j: i32,
}

impl Coord {
    fn new() -> Coord {
        Coord { i: 0, j: 0 }
    }
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, other: Coord) -> Coord {
        Coord {
            i: self.i + other.i,
            j: self.j + other.j,
        }
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, other: Coord) {
        *self = Coord {
            i: self.i + other.i,
            j: self.j + other.j,
        };
    }
}

impl Rope {
    fn new(size: usize) -> Self {
        Rope {
            v: vec![Coord::new(); size],
        }
    }
    fn step(&mut self, dir: &Direction) {
        self.v[0] += match dir {
            Direction::U => Coord { i: -1, j: 0 },
            Direction::D => Coord { i: 1, j: 0 },
            Direction::L => Coord { i: 0, j: -1 },
            Direction::R => Coord { i: 0, j: 1 },
        };

        for i in 0..self.v.len() - 1 {
            let (left, right) = self.v.split_at_mut(i + 1);
            let h = &mut left[i];
            let t = &mut right[0];

            Rope::step_segment(h, t);
        }
    }

    fn step_segment(h: &mut Coord, t: &mut Coord) {
        let dist_i = (h.i - t.i).abs();
        let dir_i = match h.i - t.i {
            0 => 0,
            diff => diff / diff.abs(),
        };

        let dist_j = (h.j - t.j).abs();
        let dir_j = match h.j - t.j {
            0 => 0,
            diff => diff / diff.abs(),
        };

        let step = match dist_i + dist_j {
            0 | 1 => Coord { i: 0, j: 0 },
            2 => match dist_i {
                0 => Coord { i: 0, j: dir_j },
                1 => Coord { i: 0, j: 0 },
                2 => Coord { i: dir_i, j: 0 },
                _ => {
                    panic!("unexpected values");
                }
            },
            3 | 4 => Coord { i: dir_i, j: dir_j },
            _ => panic!("unexpected value"),
        };

        *t += step;
    }

    fn tail(&self) -> &Coord {
        self.v.last().unwrap()
    }

    #[allow(dead_code)]
    fn head(&self) -> &Coord {
        self.v.first().unwrap()
    }
}

impl Direction {
    fn parse_move(s: &String) -> (Direction, u32) {
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

    let moves: Vec<(Direction, u32)> = lines.iter().map(Direction::parse_move).collect();

    let mut rope = Rope::new(2);
    println!("part 1: {}", move_rope(&mut rope, &moves));

    let mut rope = Rope::new(10);
    println!("part 2: {}", move_rope(&mut rope, &moves));
}

fn move_rope(rope: &mut Rope, moves: &Vec<(Direction, u32)>) -> usize {
    let mut tail_positions = HashSet::<Coord>::new();

    tail_positions.insert(*rope.tail());

    for (dir, steps) in moves {
        for _ in 0..*steps {
            rope.step(&dir);
            tail_positions.insert(*rope.tail());
        }
    }

    tail_positions.len()
}
