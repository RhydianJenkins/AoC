mod days;

use std::env;

fn execute_day(day: usize) -> Result<&'static str, &'static str> {
    match day {
        1 => Ok(days::day1::solve()),
        2 => Ok(days::day2::solve()),
        3 => Ok(days::day3::solve()),
        4 => Ok(days::day4::solve()),
        5 => Ok(days::day5::solve()),
        6 => Ok(days::day6::solve()),
        7 => Ok(days::day7::solve()),
        8 => Ok(days::day8::solve()),
        9 => Ok(days::day9::solve()),
        10 => Ok(days::day10::solve()),
        11 => Ok(days::day11::solve()),
        12 => Ok(days::day12::solve()),
        13 => Ok(days::day13::solve()),
        14 => Ok(days::day14::solve()),
        15 => Ok(days::day15::solve()),
        16 => Ok(days::day16::solve()),
        17 => Ok(days::day17::solve()),
        18 => Ok(days::day18::solve()),
        19 => Ok(days::day19::solve()),
        20 => Ok(days::day20::solve()),
        21 => Ok(days::day21::solve()),
        22 => Ok(days::day22::solve()),
        23 => Ok(days::day23::solve()),
        24 => Ok(days::day24::solve()),
        25 => Ok(days::day25::solve()),
        _ => Err("Oh no"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<usize>().unwrap();
    let output = execute_day(day).unwrap();

    println!("{}", output);
}
