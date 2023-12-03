mod game;

use game::Game;

fn get_id(line: &str) -> u8 {
    let id = line
        .replace("Game ", "")
        .split(":")
        .next()
        .unwrap()
        .parse::<u8>()
        .unwrap();

    println!("ID: {}", id);

    id
}

fn read_games(inputs: Vec<&str>) -> Result<Vec<Game>, &str> {
    let games = inputs
        .iter()
        .map(|line| {
            let id = get_id(line);
            let rest = line.split(";").next().unwrap();
            let _counts = rest.split(";").next().unwrap();

            Game::new(id, 1, 2, 3)
        })
        .collect();

    Ok(games)
}

pub fn solve() -> Result<String, String> {
    let _inputs: Vec<&str> = include_str!("./input.txt").lines().collect();
    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_games_returns_ok() {
        let result = read_games(vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ]);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 5);
    }
}
