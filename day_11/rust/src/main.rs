use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    op: Vec<String>,
    test: u64,
    next: (usize, usize),
    inspected: u64,
}

impl Monkey {
    fn new(items: Vec<u64>, op: Vec<String>, test: u64, next: (usize, usize)) -> Self {
        Monkey {
            items,
            op,
            test,
            next,
            inspected: 0,
        }
    }

    fn throw_items(&mut self, monkey_mod: u64) -> (Vec<u64>, Vec<u64>) {
        let mut true_items: Vec<u64> = vec![];
        let mut false_items: Vec<u64> = vec![];

        for i in &self.items {
            self.inspected += 1;

            let y = if self.op[1] == "old" {
                *i
            } else {
                self.op[1].parse().unwrap()
            };
            let op_result = if self.op[0] == "+" { *i + y } else { *i * y };
            // Part 1
            // let worry = (op_result as f32 / 3.0) as u64;

            // Part 2
            let worry = op_result % monkey_mod;

            if worry % self.test == 0 {
                true_items.push(worry);
            } else {
                false_items.push(worry);
            }
        }
        self.items.clear();
        (true_items, false_items)
    }
}

fn main() {
    let mut args = env::args();
    let fname = args.nth(1).expect("Provide input file as first argument");
    let file = File::open(fname).expect("File should exist.");
    let reader = BufReader::new(file);

    let mut monkeys: Vec<Monkey> = vec![];
    let mut items: Vec<u64> = vec![];
    let mut test = 0;
    let mut next = (0, 0);
    let mut operations = vec![String::new(); 2];

    for line_ in reader.lines() {
        let line = line_.unwrap();

        let split: Vec<_> = line.trim().split(' ').collect();
        match split[0] {
            "Starting" => {
                for i in &split[2..] {
                    items.push(i.replace(",", "").parse().unwrap())
                }
            }
            "Operation:" => {
                operations[0] = String::from(split[4]);
                operations[1] = String::from(split[5]);
            }
            "Test:" => test = split[3].parse().unwrap(),
            "If" => match split[1] {
                "true:" => next.0 = split[5].parse().unwrap(),
                "false:" => {
                    next.1 = split[5].parse().unwrap();
                    monkeys.push(Monkey::new(items.clone(), operations.clone(), test, next));
                    items.clear();
                }
                _ => panic!(),
            },
            _ => continue,
        }
    }

    let monkey_mod = monkeys.iter().map(|monkey| monkey.test).product();

    // let mut counter = 20;  // Part 1
    let mut counter = 10_000; // Part 2
    while counter > 0 {
        for i in 0..monkeys.len() {
            let (mut true_items, mut false_items) = monkeys[i].throw_items(monkey_mod);
            let to_true = monkeys[i].next.0;
            let to_false = monkeys[i].next.1;
            monkeys[to_true].items.append(&mut true_items);
            monkeys[to_false].items.append(&mut false_items);
        }
        counter -= 1;
    }

    monkeys.sort_by(|a, b| b.inspected.cmp(&a.inspected));
    let monkey_business = monkeys[0].inspected * monkeys[1].inspected;
    println!("{monkey_business}");
}
