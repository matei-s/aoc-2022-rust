use std::{
    collections::{HashMap, HashSet},
    io::stdin,
    time::Instant,
};

type SBPair = (i32, i32, i32, i32);

fn main() {
    let start_input_parsing = Instant::now();
    let lines: Vec<String> = stdin().lines().filter_map(Result::ok).collect();
    let y_line: i32 = lines[0].parse().unwrap();
    let search_max: i32 = lines[1].parse().unwrap();
    let coordinates = parse_coordinates(&lines[2..]);
    let duration_input_parsing = start_input_parsing.elapsed();

    let start_part1 = Instant::now();
    let position_count = compute_impossible_positions(&coordinates, y_line);
    let duration_part1 = start_part1.elapsed();
    println!("input parsing: {:?}", duration_input_parsing);
    println!();

    println!("part 1: {position_count}");
    println!("{:?}", duration_part1);
    println!();

    let start_part2 = Instant::now();
    let pos = find_possible_position_v2(&coordinates, search_max);
    let encoding = pos.0 as u64 * 4000000 + pos.1 as u64;
    let duration_part2 = start_part2.elapsed();
    println!("part 2: {encoding}");
    println!("{:?}", duration_part2);
}

fn parse_coordinates(lines: &[String]) -> Vec<SBPair> {
    let mut coordinates = Vec::<SBPair>::new();

    for line in lines.iter() {
        let tokens: Vec<i32> = line
            .replace("Sensor at x=", "")
            .replace(": closest beacon is at x=", " ")
            .replace(", y=", " ")
            .split_whitespace()
            .map(|t| t.parse().unwrap())
            .collect();

        let sb_pair = (tokens[0], tokens[1], tokens[2], tokens[3]);
        coordinates.push(sb_pair);
    }

    coordinates
}

fn compute_impossible_positions(coordinates: &Vec<SBPair>, y_line: i32) -> i32 {
    let mut x_min = i32::MAX;
    let mut x_max = i32::MIN;

    let mut occupied_positions = HashSet::<(i32, i32)>::new();

    for coordinate in coordinates {
        let (xs, ys, xb, yb) = coordinate;
        let dist = distance(coordinate);
        if (ys - y_line).abs() <= dist {
            let delta = dist - (ys - y_line).abs();

            x_min = x_min.min(xs - delta);
            x_max = x_max.max(xs + delta);
        }

        if *ys == y_line {
            occupied_positions.insert((*xs, *ys));
        }
        if *yb == y_line {
            occupied_positions.insert((*xb, *yb));
        }
    }

    let mut possible_positions = 0;

    let mut x = x_min;
    'walk: while x <= x_max {
        for (xs, ys, xb, yb) in coordinates.iter() {
            let sensor_to_point = distance(&(*xs, *ys, x, y_line));
            let sensor_to_beacon = distance(&(*xs, *ys, *xb, *yb));
            if sensor_to_point <= sensor_to_beacon {
                let jump = sensor_to_beacon - (y_line - ys).abs() + xs - x + 1;
                x += jump;
                continue 'walk;
            }
        }

        possible_positions += 1;
        x += 1;
    }

    x_max - x_min + 1 - possible_positions - occupied_positions.len() as i32
}

fn find_possible_position(coordinates: &Vec<SBPair>, search_max: i32) -> (i32, i32) {
    for y in 0..=search_max {
        let mut x = 0;
        'x_walk: while x <= search_max {
            for (xs, ys, xb, yb) in coordinates.iter() {
                let sensor_to_point = distance(&(*xs, *ys, x, y));
                let sensor_to_beacon = distance(&(*xs, *ys, *xb, *yb));
                if sensor_to_point <= sensor_to_beacon {
                    let jump = sensor_to_beacon - (y - ys).abs() + xs - x + 1;
                    x += jump;
                    continue 'x_walk;
                }
            }

            return (x, y);
        }
    }

    panic!("point not found");
}

fn find_possible_position_v2(coordinates: &Vec<SBPair>, search_max: i32) -> (i32, i32) {
    let mut sensor_ranges = HashMap::<(i32, i32), i32>::new();

    for coordinate in coordinates.iter() {
        sensor_ranges.insert((coordinate.0, coordinate.1), distance(coordinate));
    }

    let sr_vec: Vec<((i32, i32), i32)> = sensor_ranges.clone().into_iter().collect();

    for (i, ((x1, y1), d1)) in sr_vec.iter().enumerate() {
        for ((x2, y2), d2) in sr_vec[0..].iter() {
            let ds = distance(&(*x1, *y1, *x2, *y2));
            if x1 < x2 && y1 < y2 && (d1 + d2 == ds - 2 || d1 + d2 == ds - 3) {
                println!("found UL-DR pair: ({x1}, {y1}: {d1}), ({x2}, {y2}: {d2})");
            }
            if x1 > x2 && y1 > y2 && (d1 + d2 == ds - 2 || d1 + d2 == ds - 3) {
                println!("found UR-DL pair: ({x1}, {y1}: {d1}), ({x2}, {y2}: {d2})");
            }
        }
    }

    (0, 0)
}

fn distance((xs, ys, xb, yb): &SBPair) -> i32 {
    (xs - xb).abs() + (ys - yb).abs()
}
