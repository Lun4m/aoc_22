use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn bfs(
    heightmap: &Vec<Vec<u32>>,
    root: (usize, usize),
    destination: (usize, usize),
) -> Option<u32> {
    let mut explored: HashSet<(usize, usize)> = HashSet::from([root]);
    let mut queue: VecDeque<((usize, usize), u32)> = VecDeque::from([(root, 0)]);

    let right_border: usize = heightmap[0].len() - 1;
    let bottom_border: usize = heightmap.len() - 1;
    let directions = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];
    while !queue.is_empty() {
        let (point, distance) = queue.pop_front().unwrap();
        if point == destination {
            return Some(distance);
        }

        for d in &directions {
            let neighbor = match d {
                Direction::Up => {
                    if point.0 == 0 {
                        continue;
                    }
                    (point.0 - 1, point.1)
                }
                Direction::Down => {
                    if point.0 == bottom_border {
                        continue;
                    }
                    (point.0 + 1, point.1)
                }
                Direction::Left => {
                    if point.1 == 0 {
                        continue;
                    }
                    (point.0, point.1 - 1)
                }
                Direction::Right => {
                    if point.1 == right_border {
                        continue;
                    }
                    (point.0, point.1 + 1)
                }
            };
            if heightmap[neighbor.0][neighbor.1] <= heightmap[point.0][point.1] + 1
                && !explored.contains(&neighbor)
            {
                explored.insert(neighbor);
                queue.push_back((neighbor, distance + 1));
            }
        }
    }
    None
}

fn main() {
    let mut args = env::args();
    let fname = args.nth(1).expect("Provide input file as first argument");
    let file = File::open(fname).expect("File should ecolist.");
    let reader = BufReader::new(file);

    let mut heightmap: Vec<Vec<u32>> = vec![];
    let mut start_point = (0, 0);
    let mut destination = (0, 0);

    for (i, line_) in reader.lines().enumerate() {
        let line = line_.unwrap();
        heightmap.push(vec![]);
        for (j, c) in line.chars().enumerate() {
            if c == 'S' {
                start_point = (i, j);
                heightmap[i].push(0);
                continue;
            }
            if c == 'E' {
                destination = (i, j);
                heightmap[i].push(25);
                continue;
            }
            heightmap[i].push(c.to_digit(36).unwrap() - 10);
        }
    }

    println!("Destination: {:?}", destination);

    // Part 1
    let n_steps = bfs(&heightmap, start_point, destination).unwrap();
    println!("Part 1: {}", n_steps);

    // Part 2
    let mut shortest_path = 1000;
    for i in 0..heightmap.len() {
        for j in 0..heightmap[0].len() {
            if heightmap[i][j] == 0 {
                let n_steps = match bfs(&heightmap, (i, j), destination) {
                    Some(n_steps) => n_steps,
                    None => continue,
                };
                if n_steps < shortest_path {
                    shortest_path = n_steps;
                }
            }
        }
    }
    println!("Part 2: {}", shortest_path);
}
