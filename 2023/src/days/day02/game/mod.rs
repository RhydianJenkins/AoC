#[derive(Debug)]
pub struct Game {
    pub id: u64,
    max_num_red: u8,
    max_num_green: u8,
    max_num_blue: u8,
}

fn get_id(line: &str) -> Result<u64, &'static str> {
    let error_msg = "Could not extract id";

    match line.split(" ").nth(1) {
        Some(id) => Ok(id.replace(":", "").parse::<u64>().map_err(|_| error_msg)?),
        None => Err(error_msg),
    }
}

fn get_max(line: &str, color: &str) -> u8 {
    line.split(":")
        .last()
        .unwrap()
        .split(";")
        .map(|part| {
            part.split(",")
                .filter(|part| part.contains(color))
                .map(|part| {
                    part.replace(color, "")
                        .replace(" ", "")
                        .parse::<u8>()
                        .unwrap()
                })
                .next()
                .unwrap_or(0)
        })
        .max()
        .unwrap()
}

impl Game {
    pub fn new(line: &str) -> Game {
        let id = get_id(line).unwrap();
        let max_num_red = get_max(line, "red");
        let max_num_green = get_max(line, "green");
        let max_num_blue = get_max(line, "blue");

        Game {
            id,
            max_num_red,
            max_num_green,
            max_num_blue,
        }
    }

    #[allow(dead_code)]
    pub fn is_valid(
        &self,
        num_red_allowed: u8,
        num_green_allowed: u8,
        num_blue_allowed: u8,
    ) -> bool {
        self.max_num_red <= num_red_allowed
            && self.max_num_green <= num_green_allowed
            && self.max_num_blue <= num_blue_allowed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracting_id_gracefully_fails() {
        let input = "Some bs string";

        assert_eq!(get_id(input), Err("Could not extract id"))
    }

    #[test]
    fn can_extract_id() {
        let input = "Game 123: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        assert_eq!(get_id(input).unwrap(), 123);
    }

    #[test]
    fn can_extract_max_colours() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        assert_eq!(get_max(input, "red"), 4);
        assert_eq!(get_max(input, "green"), 2);
        assert_eq!(get_max(input, "blue"), 6);
    }
}
