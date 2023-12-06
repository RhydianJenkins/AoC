mod game;

use game::Game;

fn read_games(inputs: Vec<&str>) -> Result<Vec<Game>, &str> {
    let games = inputs.iter().map(|input| Game::new(input)).collect();

    Ok(games)
}

fn get_valid_games(
    games: Vec<Game>,
    allowed_red: u8,
    allowed_green: u8,
    allowed_blue: u8,
) -> Vec<Game> {
    games
        .into_iter()
        .filter(|game| game.is_valid(allowed_red, allowed_green, allowed_blue))
        .collect()
}

fn count_valid_games(games: &Vec<Game>) -> u64 {
    let val = games.iter().fold(0, |acc, game| game.id + acc);

    val
}

fn count_game_powers(games: &Vec<Game>) -> u64 {
    let power = games.iter().fold(0, |acc, game| {
        let power = game.max_num_red as u64 * game.max_num_green as u64 * game.max_num_blue as u64;

        power + acc
    });

    power
}

pub fn solve() -> Result<String, String> {
    let inputs: Vec<&str> = include_str!("./input.txt").lines().collect();
    let games = read_games(inputs)?;
    // let valid_games = get_valid_games(&games, 12, 13, 14);
    // let game_id_counts = count_valid_games(&valid_games);
    let game_power_totals = count_game_powers(&games);

    Ok(game_power_totals.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_powers() {
        let games = read_games(vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ]);
        let power = count_game_powers(&games.unwrap());
        assert_eq!(power, 2286);
    }

    #[test]
    fn reads_games_returns_ok() {
        let games = read_games(vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ]);

        assert!(games.is_ok());
        assert_eq!(games.unwrap().len(), 5);
    }

    #[test]
    fn can_filter_valid_games() {
        let games = read_games(vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ]);

        let valid_games = get_valid_games(games.unwrap(), 12, 13, 14);

        assert_eq!(valid_games.len(), 3);
    }

    #[test]
    fn it_can_count_the_ids() {
        let games = read_games(vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ]);

        let valid_games = get_valid_games(games.unwrap(), 12, 13, 14);

        let counted = count_valid_games(&valid_games);

        assert_eq!(counted, 8);
    }
}
