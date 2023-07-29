use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct CPU {
    register: i32,
    cycle: i32,
}

impl CPU {
    fn new() -> Self {
        CPU {
            register: 1,
            cycle: 1,
        }
    }

    fn run(&mut self, n_cycles: i32, value: i32, signal_stregth: &mut Vec<i32>) {
        for _ in 0..n_cycles {
            if (self.cycle - 20) % 40 == 0 {
                signal_stregth.push(self.cycle * self.register);
            }

            self.draw();
            self.cycle += 1;
        }
        self.register += value;
    }

    fn draw(&self) {
        let col = (self.cycle - 1) % 40;
        if col <= self.register + 1 && col >= self.register - 1 {
            print!("#")
        } else {
            print!(".")
        }
        if col == 39 {
            println!()
        }
    }
}

fn main() {
    let mut args = env::args();
    let fname = args.nth(1).expect("Provide input file as first argument");
    let file = File::open(fname).expect("File shoulf exist.");
    let reader = BufReader::new(file);

    let mut cpu = CPU::new();
    let mut signal_stregth = vec![];

    println!();
    for line_ in reader.lines() {
        let line = line_.unwrap();
        let instruction = &line[..4];
        let (n_cycles, value) = match instruction {
            "noop" => (1, 0),
            "addx" => (2, line[5..].trim().parse::<i32>().unwrap()),
            _ => panic!("Invalid instruction."),
        };
        cpu.run(n_cycles, value, &mut signal_stregth);
    }

    let result: i32 = signal_stregth.iter().sum();
    println!("\nPart 1: {}", result);
}
