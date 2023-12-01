mod days;

use std::env;

fn execute_day(day: usize) -> Result<&'static str, &'static str> {
    match day {
        1 => days::day01::solve(),
        _ => Err("Invalid day"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<usize>().unwrap();
    let result = execute_day(day);
    println!("{}", result.unwrap());
}
