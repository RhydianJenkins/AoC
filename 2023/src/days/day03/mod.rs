mod number;

use number::Number;
use std::num::ParseIntError;

fn read_number(input: &str, start_index: Option<usize>) -> Result<Number, ParseIntError> {
    let start_index = start_index.unwrap_or(0);
    let mut start: usize = 0;
    let mut end: usize = 0;

    for (index, digit) in input.chars().enumerate().skip(start_index) {
        if !digit.is_digit(10) {
            break;
        }

        if start == 0 {
            start = index;
        }

        end = index;
    }

    let value = input[start as usize..end as usize].parse::<usize>()?;
    let number = Number { value, start, end };

    Ok(number)
}

fn read_numbers(inputs: Vec<&str>) -> Result<Vec<Number>, String> {
    let numbers: Vec<Number> = inputs
        .iter()
        .enumerate()
        .map(|(index, input)| read_number(input, Option::from(index)).unwrap())
        .collect();

    Ok(numbers)
}

pub fn solve() -> Result<String, String> {
    let inputs: Vec<&str> = include_str!("./input.txt").lines().collect();
    let _numbers = read_numbers(inputs)?;
    Ok("".to_string())
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn reads_number() {
        let input = "467..114..";
        let number = read_number(input, Option::None);
        assert!(number.is_ok());
        assert!(number.unwrap().value == 467);
    }

    #[test]
    #[ignore]
    fn reads_numbers() {
        let input = vec![
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ];

        let numbers = read_numbers(input);
        assert!(numbers.is_ok());
        // assert!(numbers.unwrap().len() == 10);
    }
}
