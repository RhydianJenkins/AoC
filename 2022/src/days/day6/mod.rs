use std::fs::read_to_string;

pub fn solve() -> &'static str {
    let input = read_to_string("./src/days/day6/input.txt").unwrap();
    let raw_input: Vec<&str> = input.split("\n").filter(|x| !x.is_empty()).collect();

    // println!("Day 6, Part 1: {}", part1(&raw_input, 4));
    println!("Day 6, Part 2: {}", part1(&raw_input, 14));

    "Day six solution"
}

pub fn part1(input: &Vec<&str>, marker_start: usize) -> usize {
    let mut total_index = 0;

    for line in input {
        let char_iterator = line.chars().enumerate();

        for i in marker_start..char_iterator.count() {
            let mut prev_letters = vec![
                line.chars().nth(i - 13).unwrap(),
                line.chars().nth(i - 12).unwrap(),
                line.chars().nth(i - 11).unwrap(),
                line.chars().nth(i - 10).unwrap(),
                line.chars().nth(i - 9).unwrap(),
                line.chars().nth(i - 8).unwrap(),
                line.chars().nth(i - 7).unwrap(),
                line.chars().nth(i - 6).unwrap(),
                line.chars().nth(i - 5).unwrap(),
                line.chars().nth(i - 4).unwrap(),
                line.chars().nth(i - 3).unwrap(),
                line.chars().nth(i - 2).unwrap(),
                line.chars().nth(i - 1).unwrap(),
                line.chars().nth(i).unwrap(),
            ];

            prev_letters.sort();
            prev_letters.dedup();

            if prev_letters.len() == marker_start {
                total_index += i + 1;
                break;
            }
        };
    }

    total_index
}
