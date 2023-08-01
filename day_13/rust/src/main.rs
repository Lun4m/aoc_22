use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Entry {
    Int(u8),
    List(Vec<Entry>),
}

fn parse_string(s: &str, mut idx: usize, parsed: &mut Vec<Entry>) -> (Vec<Entry>, usize) {
    let mut number = String::new();
    for (i, c) in s.chars().enumerate() {
        if i < idx {
            continue;
        }
        if c == '[' {
            let mut inner: Vec<Entry> = vec![];
            (inner, idx) = parse_string(s, i + 1, &mut inner);
            parsed.push(Entry::List(inner));
        } else if c == ']' {
            if !number.is_empty() {
                parsed.push(Entry::Int(number.parse::<u8>().unwrap()));
            }
            return (parsed.clone(), i + 1);
        } else if c == ',' {
            if !number.is_empty() {
                parsed.push(Entry::Int(number.parse::<u8>().unwrap()));
                number.clear();
            }
        } else {
            number.push(c);
        }
    }
    (parsed.clone(), idx)
}

fn compare(first: &Vec<Entry>, second: &Vec<Entry>) -> Ordering {
    for (left, right) in first.iter().zip(second.iter()) {
        let out: Ordering = match (left, right) {
            (Entry::Int(a), Entry::Int(b)) => {
                if a == b {
                    Ordering::Equal
                } else if a < b {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
            (Entry::List(a), Entry::List(b)) => compare(a, b),
            (Entry::List(a), Entry::Int(b)) => compare(a, &vec![Entry::Int(*b)]),
            (Entry::Int(a), Entry::List(b)) => compare(&vec![Entry::Int(*a)], b),
        };
        return match out {
            Ordering::Less => out,
            Ordering::Greater => out,
            Ordering::Equal => continue,
        };
    }
    if first.len() == second.len() {
        Ordering::Equal
    } else if first.len() < second.len() {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

fn main() {
    let mut args = env::args();
    let fname = args.nth(1).expect("Provide input file as first argument");
    let file = File::open(fname).expect("File should ecolist.");
    let reader = BufReader::new(file);

    // Part 1
    // let mut first = String::new();
    // let mut second = String::new();
    // let mut indices: Vec<u32> = vec![];
    // let mut pair_number = 1;
    // let mut line_counter = 0;
    //
    // for line_ in reader.lines() {
    //     match line_counter {
    //         0 => first = String::from(line_.unwrap()),
    //         1 => second = String::from(line_.unwrap()),
    //         _ => {
    //             let (left, _) = parse_string(&first[1..first.len()], 0, &mut vec![]);
    //             let (right, _) = parse_string(&second[1..second.len()], 0, &mut vec![]);
    //             println!("PAIR NR {}", pair_number);
    //             match compare(&left, &right) {
    //                 Ordering::Less => indices.push(pair_number),
    //                 _ => (),
    //             }
    //             line_counter = -1;
    //             pair_number += 1;
    //         }
    //     }
    //     line_counter += 1;
    // }
    //
    // let result: u32 = indices.iter().sum();
    // println!("{}", result);

    // Part 2
    let mut packets: Vec<Vec<Entry>> = vec![
        vec![Entry::List(vec![Entry::Int(2)])],
        vec![Entry::List(vec![Entry::Int(6)])],
    ];
    for line_ in reader.lines() {
        let line = String::from(line_.unwrap());
        if line == "" {
            continue;
        }
        let (parsed, _) = parse_string(&line[1..line.len()], 0, &mut vec![]);
        packets.push(parsed);
    }

    packets.sort_by(|a, b| compare(a, b));
    // for packet in &packets {
    //     println!("{:?}", packet)
    // }

    // Probably there's a better way of doing this :/
    let result: usize = packets
        .iter()
        .enumerate()
        .map(|(i, v)| {
            if *v == vec![Entry::List(vec![Entry::Int(2)])]
                || *v == vec![Entry::List(vec![Entry::Int(6)])]
            {
                i + 1
            } else {
                1
            }
        })
        .product();
    println!("{}", result);
}
