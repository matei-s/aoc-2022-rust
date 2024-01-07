use std::io::{stdin, BufRead, BufReader};

#[derive(Copy, Clone, PartialEq, Debug)]
enum Move {
    Rock,
    Paper,
    Scissor,
}

impl Move {
    fn parse(s: &str) -> Self {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissor,
            _ => {
                panic!("invalid move");
            }
        }
    }

    fn score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissor => 3,
        }
    }

    fn wins_agains(&self) -> Move {
        match self {
            Move::Rock => Move::Scissor,
            Move::Paper => Move::Rock,
            Move::Scissor => Move::Paper,
        }
    }

    fn loses_against(&self) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissor,
            Move::Scissor => Move::Rock,
        }
    }

    fn determine_move(&self, outcome: &str) -> Move {
        match outcome {
            "X" => self.wins_agains(),
            "Y" => *self,
            "Z" => self.loses_against(),
            _ => panic!("invalid outcome"),
        }
    }

    fn play(&self, other: &Move) -> u32 {
        let score = self.score();

        if self == other {
            score + 3
        } else if self.wins_agains() == *other {
            score + 6
        } else {
            score
        }
    }
}

fn main() {
    let stdin = stdin();
    let reader = BufReader::new(stdin.lock());

    let rounds = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| {
            let moves = line.trim().split(' ').collect::<Vec<&str>>();
            (String::from(moves[0]), String::from(moves[1]))
        })
        .collect::<Vec<(String, String)>>();

    part_1(&rounds);
    part_2(&rounds);
}

fn part_1(rounds: &Vec<(String, String)>) {
    let score = rounds
        .iter()
        .map(|(m1, m2)| (Move::parse(m1), Move::parse(m2)))
        .map(|(op, elf)| elf.play(&op))
        .fold(0, |a, b| a + b);

    println!("part 1: {score}");
}

fn part_2(rounds: &Vec<(String, String)>) {
    let score = rounds
        .iter()
        .map(|(mv, out)| {
            let mv1 = Move::parse(mv);
            let mv2 = mv1.determine_move(out);
            (mv1, mv2)
        })
        .map(|(op, elf)| elf.play(&op))
        .fold(0, |a, b| a + b);

    println!("part 2: {score}");
}
