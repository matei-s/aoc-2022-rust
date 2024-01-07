use std::io::{stdin, BufRead};

#[derive(Debug, Copy, Clone)]
struct Interval {
    start: u32,
    end: u32,
}

impl Interval {
    fn parse(s: &str) -> Self {
        let numbers = s
            .split('-')
            .map(|s| s.parse::<u32>())
            .filter_map(Result::ok)
            .collect::<Vec<u32>>();

        Interval {
            start: numbers[0],
            end: numbers[1],
        }
    }

    fn contains(&self, other: &Interval) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Interval) -> bool {
        (self.start..=self.end).contains(&other.start)
            || (self.start..=self.end).contains(&other.end)
            || other.contains(self)
    }
}

fn main() {
    let lines = stdin().lock().lines().filter_map(Result::ok);

    let pairs: Vec<(Interval, Interval)> = lines
        .map(|line| {
            let intervals = line
                .split(',')
                .map(Interval::parse)
                .collect::<Vec<Interval>>();
            (intervals[0], intervals[1])
        })
        .collect();

    part_1(&pairs);
    part_2(&pairs);
}

fn part_1(pairs: &Vec<(Interval, Interval)>) {
    let complete_overlaps = pairs
        .iter()
        .filter(|(i1, i2)| i1.contains(&i2) || i2.contains(&i1))
        .count();

    println!("part 1: {complete_overlaps}");
}

fn part_2(pairs: &Vec<(Interval, Interval)>) {
    let overlaps = pairs.iter().filter(|(i1, i2)| i1.overlaps(&i2)).count();

    println!("part 1: {overlaps}");
}
