use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn draw_scan(coords: Vec<&str>, scan: &mut [[char; 170]; 150]) {
    for i in 1..coords.len() {
        let p1: Vec<_> = coords[i]
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let p2: Vec<_> = coords[i - 1]
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        if p1[0] == p2[0] {
            if p1[1] > p2[1] {
                for j in p2[1]..=p1[1] {
                    scan[p1[0] - 450][j] = '#';
                }
            } else {
                for j in p1[1]..=p2[1] {
                    scan[p1[0] - 450][j] = '#';
                }
            }
        } else {
            if p1[0] > p2[0] {
                for j in p2[0]..=p1[0] {
                    scan[j - 450][p1[1]] = '#';
                }
            } else {
                for j in p1[0]..=p2[0] {
                    scan[j - 450][p1[1]] = '#';
                }
            }
        }
    }
}

fn simulate_sand(scan: &mut [[char; 170]; 150]) -> u32 {
    let mut n_grains: u32 = 0;
    loop {
        let mut grain: [usize; 2] = [50, 0];
        loop {
            // Check down
            if ['#', 'o'].contains(&scan[grain[0]][grain[1]]) {
                // Check down-left
                if ['#', 'o'].contains(&scan[grain[0] - 1][grain[1]]) {
                    // Check down-right
                    if ['#', 'o'].contains(&scan[grain[0] + 1][grain[1]]) {
                        scan[grain[0]][grain[1] - 1] = 'o';
                        break;
                    } else {
                        grain[0] += 1;
                    }
                } else {
                    grain[0] -= 1;
                }
            } else {
                grain[1] += 1;
                if grain[1] == scan[0].len() - 1 {
                    return n_grains;
                }
            }
        }
        n_grains += 1;
    }
}

fn main() {
    let mut args = env::args();
    let fname = args.nth(1).expect("Provide input file as first argument");
    let file = File::open(fname).expect("File should exist.");
    let reader = BufReader::new(file);
    let mut scan = [['.'; 170]; 150];

    // let test_input = vec![
    //     "498,4 -> 498,6 -> 496,6",
    //     "503,4 -> 502,4 -> 502,9 -> 494,9",
    // ];

    for line_ in reader.lines() {
        let line = line_.unwrap();
        let split: Vec<_> = line.split(" -> ").collect();
        draw_scan(split, &mut scan);
    }
    // for line in &test_input {
    //     let split: Vec<_> = line.split(" -> ").collect();
    //     draw_scan(split, &mut scan);
    // }

    scan[50][0] = 'S';

    let n_grains = simulate_sand(&mut scan);
    // Print transposed array
    for col in 0..scan[0].len() {
        for row in &scan {
            print!("{}", row[col]);
        }
        println!();
    }

    // for row in &scan {
    //     for col in row {
    //         print!("{}", col);
    //     }
    //     println!();
    // }

    println!("{}", n_grains);
    // 727 too low
}
