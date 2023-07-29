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

impl Point {
    fn add(&mut self, other: &Point) {
        self.x += other.x;
        self.y += other.y;
    }

    fn norm(&self) -> Point {
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

fn update_tail(
    head: &mut Point,
    tail: &mut Point,
    visited: &mut HashSet<Point>,
    mut n_steps: i32,
    step: &Point,
) {
    while n_steps > 0 {
        head.add(step);
        // print!("{:?}\t", head);
        let d = distance(head, tail);
        if d.x.abs() >= 2 {
            if d.y == 0 {
                tail.add(step);
                visited.insert(*tail);
            } else {
                tail.add(&d.norm());
                visited.insert(*tail);
            }
        } else if d.y.abs() >= 2 {
            if d.x == 0 {
                tail.add(step);
                visited.insert(*tail);
            } else {
                tail.add(&d.norm());
                visited.insert(*tail);
            }
        }
        // println!("{:?}", tail);
        n_steps -= 1;
    }
}

fn main() {
    let fname = env::args()
        .nth(1)
        .expect("Provide 'input.txt' as first argument.");
    let f = File::open(fname).expect("File should exist");
    let reader = BufReader::new(f);

    let mut head = Point::new(0, 0);
    let mut tail = Point::new(0, 0);
    let mut visited = HashSet::<Point>::new();
    visited.insert(tail);

    // let mut counter = 0;
    // println!("{:?}\t{:?}", head, tail);
    for line_ in reader.lines() {
        // if counter == 10 {
        //     break;
        // }
        let line = line_.unwrap();
        let direction = &line[..1];
        let n_steps: i32 = line[2..].parse().unwrap();
        match direction {
            "R" => {
                let step = Point::new(1, 0);
                update_tail(&mut head, &mut tail, &mut visited, n_steps, &step);
            }
            "L" => {
                let step = Point::new(-1, 0);
                update_tail(&mut head, &mut tail, &mut visited, n_steps, &step);
            }
            "U" => {
                let step = Point::new(0, 1);
                update_tail(&mut head, &mut tail, &mut visited, n_steps, &step);
            }
            "D" => {
                let step = Point::new(0, -1);
                update_tail(&mut head, &mut tail, &mut visited, n_steps, &step);
            }
            _ => panic!("Unrecognized direction: {direction}"),
        }
        // counter += 1;
    }
    // println!("Final set with len {}: {:?}", visited.len(), visited);
    println!("Final set with len {}", visited.len());
}
