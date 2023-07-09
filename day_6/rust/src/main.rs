use std::{collections::HashSet, fs};

fn main() {
    let line = fs::read_to_string("../input.txt").expect("Unable to read file");
    let len = 14;

    for i in 0..(line.len() - len) {
        let slice = &line[i..(i + len)];
        let set: HashSet<char> = slice.chars().collect();
        if set.len() == len {
            println!("{}", i + len);
            return;
        }
    }
}
