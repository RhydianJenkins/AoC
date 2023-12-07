#[derive(Debug)]
#[allow(dead_code)]
pub struct Game {
    pub id: u64,
    pub min_num_red: u8,
    pub min_num_green: u8,
    pub min_num_blue: u8,
    pub max_num_red: u8,
    pub max_num_green: u8,
    pub max_num_blue: u8,
}

fn get_id(line: &str) -> Result<u64, &'static str> {
    let error_msg = "Could not extract id";

    match line.split(' ').nth(1) {
        Some(id) => Ok(id.replace(':', "").parse::<u64>().map_err(|_| error_msg)?),
        None => Err(error_msg),
    }
}

fn get_min_cubes_possible(line: &str) -> (u8, u8, u8) {
    let rounds = line.split([':', ';']).skip(1);
    let mut largest_round = (0, 0, 0);
    let mut largest_total = 0;
    let mut num_red = 0;
    let mut num_green = 0;
    let mut num_blue = 0;

    for round in rounds {
        let mut total = 0;

        for colour in round.split(',') {
            let mut split = colour.split(' ').skip(1);
            let num = split.next().unwrap().parse::<u8>().unwrap();

            match split.next().unwrap() {
                "red" => num_red = num,
                "green" => num_green = num,
                "blue" => num_blue = num,
                _ => println!("Unrecognised colour '{}'", colour),
            }

            total += num;
        }

        if total > largest_total {
            largest_round = (num_red, num_green, num_blue);
            largest_total = total;
        }
    }

    return largest_round;
}

fn get_max(line: &str, color: &str) -> u8 {
    line.split(':')
        .last()
        .unwrap()
        .split(';')
        .map(|part| {
            part.split(',')
                .filter(|part| part.contains(color))
                .map(|part| {
                    part.replace(color, "")
                        .replace(' ', "")
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
        let (min_num_red, min_num_green, min_num_blue) = get_min_cubes_possible(line);
        let max_num_red = get_max(line, "red");
        let max_num_green = get_max(line, "green");
        let max_num_blue = get_max(line, "blue");

        Game {
            id,
            min_num_red,
            min_num_green,
            min_num_blue,
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

    #[test]
    fn can_find_smallest_game() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let (min_red, min_green, min_blue) = get_min_cubes_possible(input);

        assert_eq!(min_red, 1);
        assert_eq!(min_green, 2);
        assert_eq!(min_blue, 6);
    }
}
