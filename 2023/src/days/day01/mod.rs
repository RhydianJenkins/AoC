use std::fs::read_to_string;

pub fn get_number(input: &str) -> Result<usize, &'static str> {
    let input = input.chars().collect::<Vec<char>>();
    let mut first: Option<usize> = Option::None;
    let mut last: Option<usize> = Option::None;

    let mut index = 0;

    while index < input.len() {
        if input[index].is_numeric() {
            if first == Option::None {
                first = Some(input[index].to_digit(10).unwrap() as usize)
            }
            last = Some(input[index].to_digit(10).unwrap() as usize)
        }

        index += 1;
    }

    if first == Option::None || last == Option::None {
        return Err("No number found");
    }

    let number_string = first.unwrap().to_string() + &last.unwrap().to_string();
    let number = number_string.parse::<usize>().unwrap();

    Ok(number)
}

pub fn calculate_sums(inputs: Vec<&str>) -> Result<usize, String> {
    let sums = inputs.iter().fold(Ok(0), |acc, input| {
        let num = get_number(input)?;
        acc.map(|acc| acc + num)
    });

    sums
}

pub fn solve() -> Result<String, String> {
    let inputs = include_str!("./input.txt");
    println!("Day 1 Part 1: {}", inputs);
    let result = calculate_sums(inputs.lines().collect()).unwrap();
    Ok(result.to_string())
}

pub mod tests {
    use super::*;

    #[test]
    fn no_number_should_error() {
        let test_input = "abcdef";
        let answer = get_number(test_input);

        assert_eq!(answer, Err("No number found"));
    }

    #[test]
    fn with_example_input() {
        let test_input = "ab1cd2ef";
        let answer = get_number(test_input);

        assert_eq!(answer, Ok(12));
    }

    #[test]
    fn with_example_inputs() {
        let test_input: Vec<&str> = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        let answer = calculate_sums(test_input);

        assert_eq!(answer, Ok(142));
    }
}
