use std::{fs::read_to_string, num::ParseIntError};

pub fn solve() -> &'static str {
    let input = read_to_string("./src/days/day1/input.txt").unwrap();
    let elves = input.split("\n\n");

    for elf in elves {
        let elf_calories: Vec<Result<i32, ParseIntError>> = elf
            .split("\n")
            .map(|x| x.parse::<i32>())
            .collect();

        let mut total = 0;

        for calorie in elf_calories {
            if calorie.is_ok() {
                total = total + calorie.unwrap();
            }
        }

        println!("{}", total);
    }

    return "Day one solution";
}
