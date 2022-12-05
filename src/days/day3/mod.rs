use std::fs::read_to_string;

pub fn solve() -> &'static str {
    let input = read_to_string("./src/days/day3/input.txt").unwrap();
    let bags: Vec<&str> = input.split("\n").filter(|x| !x.is_empty()).collect();
    let mut total_score = 0;

    for bag in &bags {
        let first_half = bag.chars().take(bag.len() / 2).collect::<Vec<_>>();
        let second_half = bag.chars().skip(bag.len() / 2).collect::<Vec<_>>();
        let common_char = get_common_char(&first_half, &second_half);

        let ascii = *common_char as u8;

        total_score = total_score + get_char_priority_score(ascii);
    }

    println!("Total priority: {}", total_score);

    "Day three solution"
}

fn get_common_char<'a>(first: &'a Vec<char>, second: &'a Vec<char>) -> &'a char {
    for x in first {
        for y in second {
            if x == y {
                return x;
            }
        }
    };

    &' '
}

fn get_char_priority_score(ascii: u8) -> usize {
    if ascii >= 65 && ascii <= 90 {
        return ascii as usize - 64 + 26;
    }

    if ascii >= 97 && ascii <= 122 {
        return ascii as usize - 96;
    }

    println!("Invalid char: {}", ascii);
    0
}
