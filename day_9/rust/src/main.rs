use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rope {
    n_knots: usize,
    knots: Vec<Point>,
}

impl Rope {
    fn new(n_knots: usize) -> Self {
        Rope {
            n_knots,
            knots: vec![Point::new(0, 0); n_knots],
        }
    }

    fn update_knots(&mut self, visited: &mut HashSet<Point>, mut n_steps: i32, step: Point) {
        while n_steps > 0 {
            n_steps -= 1;
            self.knots[0].add(step);
            for i in 1..self.n_knots {
                let d = distance(&self.knots[i - 1], &self.knots[i]);

                if d.x.abs() >= 2 || d.y.abs() >= 2 {
                    self.knots[i].add(d.normalize());
                    if i == self.n_knots - 1 {
                        visited.insert(self.knots[i]);
                    }
                } else {
                    // skip the other knots if the distance is short
                    break;
                }
            }
        }
    }
}

impl Point {
    fn add(&mut self, other: Point) {
        self.x += other.x;
        self.y += other.y;
    }

    fn normalize(&self) -> Point {
        let (mut x, mut y) = (0, 0);
        if self.x != 0 {
            x = self.x / self.x.abs();
        }
        if self.y != 0 {
            y = self.y / self.y.abs();
        }
        Point::new(x, y)
    }

    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

fn distance(head: &Point, tail: &Point) -> Point {
    Point {
        x: (head.x - tail.x),
        y: (head.y - tail.y),
    }
}

fn main() {
    let mut args = env::args();
    let usage = "USAGE: cargo run input.txt N_KNOTS";
    let fname = args.nth(1).expect(usage);
    let n_knots: usize = args
        .nth(0) // iterator is consumed by previous call
        .expect(usage)
        .parse()
        .expect("N_KNOTS should be a positive integer.");
    let f = File::open(fname).expect("File should exist");
    let reader = BufReader::new(f);

    let mut rope = Rope::new(n_knots);
    let mut visited = HashSet::<Point>::new();
    visited.insert(rope.knots[0]);

    for line_ in reader.lines() {
        let line = line_.unwrap();
        let direction = &line[..1];
        let n_steps: i32 = line[2..].parse().unwrap();
        let step = match direction {
            "R" => Point::new(1, 0),
            "L" => Point::new(-1, 0),
            "U" => Point::new(0, 1),
            "D" => Point::new(0, -1),
            _ => panic!("Unrecognized direction: {direction}"),
        };
        rope.update_knots(&mut visited, n_steps, step);
    }
    println!(
        "Number of places visited by the tail is {}, for a rope with {} knots",
        visited.len(),
        rope.n_knots
    );
}
