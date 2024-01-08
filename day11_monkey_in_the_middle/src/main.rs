#![allow(dead_code)]
use std::{
    collections::HashMap,
    fmt::Debug,
    io::stdin,
    num::ParseIntError,
    ops::{Add, Div, Mul, Rem},
    str::FromStr,
};

const MAX_DIV: usize = 23;

struct Monkey<T> {
    items: Vec<T>,
    inspect_count: u32,
    op: Operation,
    test: u32,
    throw: (usize, usize),
    relief: u32,
}

#[derive(Debug)]
enum Operand {
    Mul,
    Add,
    Square,
}

#[derive(Debug)]
struct Operation {
    op: Operand,
    value: u32,
}

#[derive(Clone, Debug)]
struct NumAsMods {
    mods: Vec<u32>,
}

impl NumAsMods {
    fn new() -> NumAsMods {
        NumAsMods {
            mods: vec![0; MAX_DIV + 1],
        }
    }

    fn from(number: u32) -> NumAsMods {
        let mut result = NumAsMods::new();
        for i in 1..result.mods.len() {
            result.mods[i] = number % i as u32;
        }

        result
    }
}

impl Add<u32> for &NumAsMods {
    type Output = NumAsMods;
    fn add(self, other: u32) -> NumAsMods {
        let mut result = NumAsMods::new();

        for i in 1..self.mods.len() {
            result.mods[i] = (self.mods[i] + other) % i as u32;
        }

        result
    }
}

impl Mul<u32> for &NumAsMods {
    type Output = NumAsMods;
    fn mul(self, other: u32) -> NumAsMods {
        let mut result = NumAsMods::new();

        for i in 1..self.mods.len() {
            result.mods[i] = (self.mods[i] * other) % i as u32;
        }

        result
    }
}

impl Mul for &NumAsMods {
    type Output = NumAsMods;
    fn mul(self, other: Self) -> NumAsMods {
        let mut result = NumAsMods::new();

        for i in 1..self.mods.len() {
            result.mods[i] = (self.mods[i] * other.mods[i]) % i as u32;
        }

        result
    }
}

impl Rem<u32> for &NumAsMods {
    type Output = u32;

    fn rem(self, other: u32) -> u32 {
        self.mods[other as usize]
    }
}

// dummy
impl Div<u32> for NumAsMods {
    type Output = NumAsMods;
    fn div(self, _: u32) -> NumAsMods {
        let mut result = NumAsMods::new();
        for i in 0..=MAX_DIV {
            result.mods[i] = self.mods[i]
        }
        result
    }
}

impl FromStr for NumAsMods {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num = s.parse::<u32>()?;
        Ok(NumAsMods::from(num))
    }
}

impl<T: FromStr + Debug + Clone> Monkey<T>
where
    T::Err: Debug,
    for<'a> &'a T: Mul<&'a T, Output = T>
        + Mul<u32, Output = T>
        + Add<u32, Output = T>
        + Rem<u32, Output = u32>,
    T: Div<u32, Output = T>,
{
    fn parse(lines: &[String], relief: u32) -> Monkey<T> {
        let items: Vec<T> = lines[1]
            .replace("Starting items: ", "")
            .replace(" ", "")
            .split(",")
            .map(|tok| tok.parse().unwrap())
            .collect();

        let test = lines[3].split_whitespace().last().unwrap().parse().unwrap();
        let if_true = lines[4].split_whitespace().last().unwrap().parse().unwrap();
        let if_false = lines[5].split_whitespace().last().unwrap().parse().unwrap();

        Monkey {
            items,
            inspect_count: 0,
            op: Operation::parse(&lines[2]),
            test,
            throw: (if_true, if_false),
            relief,
        }
    }

    fn execute_throw<'s>(&'s mut self) -> HashMap<usize, Vec<T>> {
        let mut thrown_items: HashMap<usize, Vec<T>> = HashMap::new();

        for value in self.items.iter() {
            let value: T = self.op.apply(&value) / self.relief;

            let throw_to = if self.test(&value) {
                self.throw.0
            } else {
                self.throw.1
            };

            self.inspect_count += 1;

            let destination_items = thrown_items.entry(throw_to).or_insert(Vec::new());
            destination_items.push(value);
        }

        self.items.clear();
        thrown_items
    }

    fn print(&self) {
        println!("starting items: {:?}", self.items);
        println!("operation: {:?}", self.op);
        println!("test: divisible by {}", self.test);
        println!("throws: {:?}", self.throw);
    }

    fn test(&self, value: &T) -> bool {
        return value % self.test == 0;
    }
}

impl Operation {
    fn parse(line: &String) -> Self {
        let raw = line.replace("Operation: new = old ", "");
        let tokens: Vec<&str> = raw.split_whitespace().collect();

        match tokens.as_slice() {
            ["*", "old"] => Self {
                value: 0,
                op: Operand::Square,
            },
            ["+", val] => Self {
                value: val.parse().unwrap(),
                op: Operand::Add,
            },
            ["*", val] => Self {
                value: val.parse().unwrap(),
                op: Operand::Mul,
            },
            tok => {
                panic!("unknown operation: {tok:?}");
            }
        }
    }

    fn apply<T: Clone>(&self, value: &T) -> T
    where
        for<'a> &'a T: Mul<&'a T, Output = T> + Mul<u32, Output = T> + Add<u32, Output = T>,
    {
        match self.op {
            Operand::Add => value + self.value,
            Operand::Mul => value * self.value,
            Operand::Square => value * value,
        }
    }
}

fn main() {
    let lines: Vec<String> = stdin().lines().filter_map(Result::ok).collect();

    let mut monkeys = parse_monkeys::<u32>(&lines, 3);
    let monkey_business = compute_monkey_business(&mut monkeys, 20);
    println!("part 1: {monkey_business}");
    println!();

    let mut monkeys = parse_monkeys::<NumAsMods>(&lines, 1);
    let monkey_business = compute_monkey_business(&mut monkeys, 10000);

    println!("part 2: {monkey_business}");
}

fn parse_monkeys<T>(lines: &Vec<String>, relief: u32) -> Vec<Monkey<T>>
where
    T: FromStr + Debug + Clone,
    T::Err: Debug,
    for<'a> &'a T: Mul<&'a T, Output = T>
        + Mul<u32, Output = T>
        + Add<u32, Output = T>
        + Rem<u32, Output = u32>,
    T: Div<u32, Output = T>,
{
    let mut monkeys = Vec::<Monkey<T>>::new();
    for chunk in lines.chunks(7) {
        monkeys.push(Monkey::parse(chunk, relief));
    }

    monkeys
}

fn compute_monkey_business<T>(monkeys: &mut Vec<Monkey<T>>, rounds: u32) -> u64
where
    T: FromStr + Debug + Clone,
    T::Err: Debug,
    for<'a> &'a T: Mul<&'a T, Output = T>
        + Mul<u32, Output = T>
        + Add<u32, Output = T>
        + Rem<u32, Output = u32>,
    T: Div<u32, Output = T>,
{
    for _ in 0..rounds {
        for m_id in 0..monkeys.len() {
            let mut thrown_items = monkeys[m_id].execute_throw();
            for (id, items) in thrown_items.iter_mut() {
                monkeys[*id].items.append(&mut items.clone());
            }
        }
    }

    for (i, monkey) in monkeys.iter().enumerate() {
        println!("monkey {i} inspected {} items", monkey.inspect_count);
    }

    monkeys.sort_by(|m1, m2| m2.inspect_count.cmp(&m1.inspect_count));

    monkeys[0].inspect_count as u64 * monkeys[1].inspect_count as u64
}
