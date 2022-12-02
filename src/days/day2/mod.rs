use std::fs::read_to_string;

pub fn solve() -> &'static str {
    let input = read_to_string("./src/days/day2/input.txt").unwrap();
    let rounds: Vec<&str> = input.split("\n").filter(|x| !x.is_empty()).collect();
    let mut total_score_part_1 = 0;
    let mut total_score_part_2 = 0;

    for round in rounds {
        let both_rock_paper_scissors = round.split(" ");
        let enemy_rps = both_rock_paper_scissors.clone().nth(0).unwrap();
        let our_rps = both_rock_paper_scissors.clone().nth(1).unwrap();

        total_score_part_1 = total_score_part_1 + calculate_score_part_1(our_rps, enemy_rps);
        total_score_part_2 = total_score_part_2 + calculate_score_part_2(our_rps, enemy_rps);
    }

    println!("Total Score Part 1: {}", total_score_part_1);
    println!("Total Score Part 2: {}", total_score_part_2);

    return "Day two solution";
}

// Enemy
// 'A' = 'Rock'
// 'B' = 'Paper'
// 'C' = 'Scissors'

// Us
// 'X' = 'Rock' + 1
// 'Y' = 'Paper' + 2
// 'Z' = 'Scissors' + 3
fn calculate_score_part_1(our_rps: &str, enemy_rps: &str) -> i32 {
    if enemy_rps == "B" {
        if our_rps == "Y" {
            return 3 + 2;
        }

        if our_rps == "Z" {
            return 6 + 3;
        }

        return 1;
    }

    if enemy_rps == "A" {
        if our_rps == "X" {
            return 3 + 1;
        }

        if our_rps == "Y" {
            return 6 + 2;
        }

        return 3;
    }

    if enemy_rps == "C" {
        if our_rps == "Z" {
            return 3 + 3;
        }

        if our_rps == "X" {
            return 6 + 1;
        }

        return 2;
    }

    println!("Something went wrong");
    return 0;
}

// Enemy
// 'A' = 'Rock' + 1
// 'B' = 'Paper' + 2
// 'C' = 'Scissors' + 3

// Us
// 'X' = lose
// 'Y' = draw
// 'Z' = win
fn calculate_score_part_2(our_rps: &str, enemy_rps: &str) -> i32 {
    if our_rps == "X" {
        return calculate_score_part_1(get_losing_move(enemy_rps), enemy_rps);
    }

    if our_rps == "Y" {
        return calculate_score_part_1(get_draw_move(enemy_rps), enemy_rps);
    }

    if our_rps == "Z" {
        return calculate_score_part_1(get_winning_move(enemy_rps), enemy_rps);
    }

    println!("Something went wrong");
    return 0;
}

fn get_losing_move(enemy_rps: &str) -> &str {
    if enemy_rps == "A" {
        return "Z";
    }

    if enemy_rps == "B" {
        return "X";
    }

    if enemy_rps == "C" {
        return "Y";
    }

    println!("Something went wrong");
    return "";
}

fn get_draw_move(enemy_rps: &str) -> &str {
    if enemy_rps == "A" {
        return "X";
    }

    if enemy_rps == "B" {
        return "Y";
    }

    if enemy_rps == "C" {
        return "Z";
    }

    println!("Something went wrong");
    return "";
}

fn get_winning_move(enemy_rps: &str) -> &str {
    if enemy_rps == "A" {
        return "Y";
    }

    if enemy_rps == "B" {
        return "Z";
    }

    if enemy_rps == "C" {
        return "X";
    }

    println!("Something went wrong");
    return "";
}
