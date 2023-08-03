use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Sensor {
    x: i32,
    y: i32,
    r: i32,
}

impl Sensor {
    fn new(x: i32, y: i32, r: i32) -> Self {
        Sensor { x, y, r }
    }

    fn get_border(&self, min: i32, max: i32) -> Vec<(i32, i32)> {
        let mut border: Vec<(i32, i32)> = vec![];
        let mut x = self.x + self.r + 1;
        let mut y = self.y;
        let dx: [i32; 4] = [-1, -1, 1, 1];
        let dy: [i32; 4] = [1, -1, -1, 1];

        for i in 0..4 {
            for _ in 0..self.r + 1 {
                if x < min || y < min || x > max || y > max {
                } else {
                    border.push((x, y));
                }
                x += dx[i];
                y += dy[i];
            }
        }
        border
    }

    fn dist_from(&self, x: i32, y: i32) -> i32 {
        (self.x - x).abs() + (self.y - y).abs()
    }
}

fn dist_from_vec(v: &Vec<i32>) -> i32 {
    (v[0] - v[2]).abs() + (v[1] - v[3]).abs()
}

fn tuning_frequncy(x: i32, y: i32) -> u64 {
    (x as u64 * 4_000_000) + y as u64
}

fn part_1(y_test: i32, sensors: &Vec<Sensor>, exclude: &Vec<i32>) -> i32 {
    let x_min = sensors.iter().map(|t| t.x).min().unwrap().clone();
    let x_max = sensors.iter().map(|t| t.x).max().unwrap().clone();
    let d_max = sensors.iter().map(|t| t.r).max().unwrap().clone();
    let mut no_beacon = 0;

    for x_test in (x_min - d_max)..=(x_max + d_max) {
        if exclude.contains(&x_test) {
            continue;
        }
        for sensor in sensors {
            if sensor.dist_from(x_test, y_test) <= sensor.r {
                no_beacon += 1;
                break;
            }
        }
    }
    no_beacon
}

fn part_2(sensors: &Vec<Sensor>) -> u64 {
    let min = 0;
    let max = 4_000_000;
    let n_sensors = sensors.len();

    for (i, sensor) in sensors.iter().enumerate() {
        for (x, y) in &sensor.get_border(min, max) {
            let mut counter = 0;
            for (j, other) in sensors.iter().enumerate() {
                if i == j {
                    continue;
                }
                if other.dist_from(*x, *y) > other.r {
                    counter += 1;
                }
            }
            if counter == n_sensors - 1 {
                return tuning_frequncy(*x, *y);
            }
        }
    }
    0
}

fn main() {
    let mut args = env::args();
    let fname = args.nth(1).expect("Provide input file as first argument");
    let file = File::open(fname).expect("File should exist.");
    let reader = BufReader::new(file);

    let y_test = 2_000_000;
    let mut sensors: Vec<Sensor> = vec![];
    let mut exclude: Vec<i32> = vec![];

    for line_ in reader.lines() {
        let ints: Vec<_> = line_
            .unwrap()
            .split(&['=', ',', ':'])
            .filter_map(|s| s.to_string().parse::<i32>().ok())
            .collect();

        if ints[1] == y_test {
            exclude.push(ints[0])
        }
        if ints[3] == y_test {
            exclude.push(ints[2])
        }
        sensors.push(Sensor::new(ints[0], ints[1], dist_from_vec(&ints)));
    }

    let result_1 = part_1(y_test, &sensors, &exclude);
    println!("Part 1: {}", result_1);

    let result_2 = part_2(&sensors);
    println!("Part 2: {}", result_2);
}
