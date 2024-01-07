use std::io::{stdin, BufRead, BufReader};

fn main() {
    let stdin = stdin();
    let reader = BufReader::new(stdin.lock());

    let mut calories: Vec<u32> = Vec::new();
    let mut item_calories = 0u32;

    reader
        .lines()
        .filter_map(|line| match line {
            Ok(l) => Some(l),
            Err(_) => None,
        })
        .for_each(|line| {
            if line.trim().len() == 0 {
                calories.push(item_calories);
                item_calories = 0;
            } else {
                item_calories += line.parse::<u32>().expect("invalid line");
            }
        });

    // push the last elf's total computed calories
    calories.push(item_calories);

    calories.sort();
    calories.reverse();

    let max = calories[0];
    println!("part 1 result: {max}");

    let top3 = calories[..3].iter().sum::<u32>();
    println!("part 2 result: {top3}");
}
