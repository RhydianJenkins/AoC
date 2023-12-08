#[derive(Debug)]
struct Number {
    pub value: u32,
    pub start: u8,
    pub end: u8,
}

fn read_numbers(inputs: Vec<&str>) -> Result<Vec<Number>, String> {
    let numbers: Vec<Number> = Vec::new();

    // go through the numbers and find the digits with their start and end indexes
    let found_numbers = inputs
        .iter()
        .map(|input| {
            let mut start: u8 = 0;
            let mut end: u8 = 0;

            for (index, digit) in input.chars().enumerate() {
                if digit.is_digit(10) {
                    if start == 0 {
                        start = index as u8;
                    }
                    end = index as u8;

                    continue;
                }

                break;
            }

            let value = input[start as usize..end as usize].parse::<u32>();

            println!("start: {:?}", start);
            println!("end: {:?}", end);

            Number {
                value: value.unwrap(),
                start,
                end,
            }
        })
        .collect::<Vec<Number>>();

    println!("Found numbers len: {}", found_numbers.len());

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
