#[derive(Debug)]
struct Number {
    pub value: u32,
    pub start: u8,
    pub end: u8,
}

fn read_numbers(inputs: Vec<&str>) -> Result<Vec<Number>, String> {
    let numbers: Vec<Number> = Vec::new();

    for input in inputs {
        let parts = input
            .split('.')
            .enumerate()
            .filter_map(|(index, c)| {
                let value = c.parse::<u32>();

                if value.is_err() {
                    return None;
                }

                let start = index as u8;
                let num_size = value.clone().unwrap().to_string().len();
                let end = index as u8 + num_size as u8;
                let num = Number {
                    value: value.unwrap(),
                    start,
                    end,
                };

                // TODO the number indexes don't account for multi-digit numbers

                Some(num)
            })
            .collect::<Vec<Number>>();
    }

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
