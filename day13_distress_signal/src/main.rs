use std::{cmp::Ordering, io::stdin};

#[derive(Debug, Clone, Eq, PartialEq)]
enum Element {
    Num(u32),
    List(Vec<Element>),
}

impl Element {
    fn parse(s: &str) -> Element {
        Element::parse_list(s.as_bytes(), 1).0
    }

    fn parse_num(s: &[u8], i: usize) -> (Element, usize) {
        let mut i = i;
        let mut n = 0;

        while s[i] >= b'0' && s[i] <= b'9' {
            n *= 10;
            n += (s[i] - b'0') as u32;
            i += 1;
        }

        (Element::Num(n), i)
    }

    fn parse_list(s: &[u8], i: usize) -> (Element, usize) {
        let mut i = i;
        let mut list = Vec::<Element>::new();

        while i < s.len() {
            match s[i] {
                b',' => i += 1,
                b'[' => {
                    let (list_el, j) = Self::parse_list(s, i + 1);
                    list.push(list_el);
                    i = j;
                }
                b']' => return (Element::List(list), i + 1),
                b'0'..=b'9' => {
                    let (num_el, j) = Self::parse_num(s, i);
                    list.push(num_el);
                    i = j;
                }
                _ => {
                    panic!("unrecognized character")
                }
            }
        }

        panic!("error reading list");
    }

    fn order(left: &Element, right: &Element) -> Ordering {
        match (left, right) {
            (Self::Num(l), Self::Num(r)) => l.cmp(r),
            (Self::List(_), Self::Num(_)) => Self::order(left, &Self::List(vec![right.clone()])),
            (Self::Num(_), Self::List(_)) => Self::order(&Self::List(vec![left.clone()]), right),
            (Self::List(l), Self::List(r)) => {
                let mut i = 0;

                while i < l.len() && i < r.len() {
                    match Self::order(&l[i], &r[i]) {
                        Ordering::Equal => i += 1,
                        ord => return ord,
                    }
                }

                l.len().cmp(&r.len())
            }
        }
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        Element::order(self, other)
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Element::order(self, other))
    }
}

fn main() {
    let lines: Vec<String> = stdin().lines().filter_map(Result::ok).collect();

    let mut result = 0;

    let mut packets = Vec::<Element>::new();

    for (i, chunk) in lines.chunks(3).enumerate() {
        let left = Element::parse(&chunk[0]);
        let right = Element::parse(&chunk[1]);

        match left < right {
            true => result += i + 1,
            _ => {}
        }

        packets.push(left);
        packets.push(right);
    }

    println!("part 1: {result}");

    let divider_1 = Element::parse("[[2]]");
    let divider_2 = Element::parse("[[6]]");

    packets.push(divider_1.clone());
    packets.push(divider_2.clone());

    packets.sort();

    let i1 = packets.iter().position(|p| *p == divider_1).unwrap() + 1;
    let i2 = packets.iter().position(|p| *p == divider_2).unwrap() + 1;

    println!("part 2: {}", i1 * i2);
}
