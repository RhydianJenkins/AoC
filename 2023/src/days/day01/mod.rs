use std::collections::HashMap;

pub fn get_number(input: &str) -> Result<usize, &str> {
    let number_map = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut first: Option<usize> = Option::None;
    let mut last: Option<usize> = Option::None;

    for index in 0..input.len() {
        for (pattern, value) in number_map.iter() {
            if input[index..].starts_with(*pattern) {
                if first == Option::None {
                    first = Some(*value);
                }
                last = Some(*value);
            }
        }
    }

    if first == Option::None {
        return Err("No number found");
    }

    let number_string = first.unwrap().to_string() + &last.unwrap().to_string();
    let number = number_string.parse::<usize>().unwrap();

    Ok(number)
}

pub fn calculate_sums(inputs: Vec<&str>) -> Result<usize, &str> {
    let sums = inputs.iter().fold(Ok(0), |acc, input| {
        let num = get_number(input)?;
        acc.map(|acc| acc + num)
    });

    sums
}

pub fn solve() -> Result<String, String> {
    let inputs = include_str!("./input.txt");
    let result = calculate_sums(inputs.lines().collect()).unwrap();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_number_should_error() {
        let test_input = "abcdef";
        let answer = get_number(test_input);

        assert_eq!(answer, Err("No number found"));
    }

    #[test]
    fn with_example_input() {
        let test_input = "ab1cdtwoef";
        let answer = get_number(test_input);

        assert_eq!(answer, Ok(12));
    }

    #[test]
    fn with_example_inputs_1() {
        let test_input: Vec<&str> = vec!["1abc2", "pqr3stueightvwx", "a1btwoc3d4e5f", "treb7uchet"];
        let answer = calculate_sums(test_input);
        assert_eq!(answer, Ok(142));
    }

    #[test]
    fn with_example_inputs_2() {
        let test_input: Vec<&str> = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];
        let answer = calculate_sums(test_input);
        assert_eq!(answer, Ok(281));
    }
}
