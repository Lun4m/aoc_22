use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut fattest_elves: Vec<i32> = [0, 0, 0].to_vec();
    let mut calories_vec: Vec<i32> = [0, 0, 0].to_vec();
    let mut total_calories: i32 = 0;
    let mut elf_num: i32 = 0;

    let file = File::open(filename).expect("Should be able to read file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let value = line.unwrap();
        if value == "" {
            if total_calories > calories_vec[2] {
                if total_calories > calories_vec[1] {
                    if total_calories > calories_vec[0] {
                        calories_vec[1] = calories_vec[0];
                        calories_vec[0] = total_calories;
                        fattest_elves[0] = elf_num;
                    } else {
                        calories_vec[2] = calories_vec[1];
                        calories_vec[1] = total_calories;
                        fattest_elves[1] = elf_num;
                    }
                } else {
                    calories_vec[2] = total_calories;
                    fattest_elves[2] = elf_num;
                }
            }
            total_calories = 0;
            elf_num += 1;
        } else {
            total_calories += value.parse::<i32>().unwrap();
        }
    }
    println!("Fattest elves: {:?}", fattest_elves);
    println!("Their calories: {:?}", calories_vec);
    println!("The total sum: {:?}", calories_vec.iter().sum::<i32>());
}
