use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn draw_scan(coords: Vec<&str>, scan: &mut [[char; 170]; 1000]) -> usize {
    let mut max_y: usize = 0;
    for i in 1..coords.len() {
        let p1: Vec<_> = coords[i]
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let p2: Vec<_> = coords[i - 1]
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        if p1[1] > max_y {
            max_y = p1[1] as usize;
        }
        if p2[1] > max_y {
            max_y = p1[1] as usize;
        }
        if p1[0] == p2[0] {
            if p1[1] > p2[1] {
                for j in p2[1]..=p1[1] {
                    scan[p1[0]][j] = '#';
                }
            } else {
                for j in p1[1]..=p2[1] {
                    scan[p1[0]][j] = '#';
                }
            }
        } else {
            if p1[0] > p2[0] {
                for j in p2[0]..=p1[0] {
                    scan[j][p1[1]] = '#';
                }
            } else {
                for j in p1[0]..=p2[0] {
                    scan[j][p1[1]] = '#';
                }
            }
        }
    }
    max_y
}

fn simulate_sand(scan: &mut [[char; 170]; 1000]) -> u32 {
    let mut n_grains: u32 = 0;
    loop {
        let mut grain: [usize; 2] = [500, 0];
        loop {
            grain[1] += 1;

            // Check abyss
            if grain[1] == scan[0].len() - 1 {
                return n_grains;
            }

            // Check down
            if ['#', '.'].contains(&scan[grain[0]][grain[1]]) {
                // Check down-left
                if ['#', '.'].contains(&scan[grain[0] - 1][grain[1]]) {
                    // Check down-right
                    if ['#', '.'].contains(&scan[grain[0] + 1][grain[1]]) {
                        // Check if starting point
                        if scan[grain[0]][grain[1] - 1] == 'S' {
                            return n_grains + 1;
                        }
                        scan[grain[0]][grain[1] - 1] = '.';
                        break;
                    } else {
                        grain[0] += 1;
                    }
                } else {
                    grain[0] -= 1;
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
    let mut scan = [[' '; 170]; 1000];

    let mut max_y: Vec<usize> = vec![];
    for line_ in reader.lines() {
        let line = line_.unwrap();
        let split: Vec<_> = line.split(" -> ").collect();
        max_y.push(draw_scan(split, &mut scan));
    }

    // Part 2, comment out the next 4 lines for part 1
    let max_y = 2 + *max_y.iter().max().unwrap();
    for row in &mut scan {
        row[max_y] = '#'
    }

    scan[500][0] = 'S';
    let n_grains = simulate_sand(&mut scan);

    // Print transposed array
    for col in 0..scan[0].len() {
        for row in &scan[450..600] {
            print!("{}", row[col]);
        }
        println!();
    }

    println!("{}", n_grains);
}
