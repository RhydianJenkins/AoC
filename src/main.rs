use std::env;

mod days;

fn execute_day(day: usize) -> String {
    return days::1::exec;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<usize>().unwrap();
    let output = execute_day(day);

    println!("Part 1: {}", output);
}
