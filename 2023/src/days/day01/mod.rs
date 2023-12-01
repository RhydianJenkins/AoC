use std::fs::read_to_string;

pub fn solve() -> Result<&'static str, &'static str> {
    let input = read_to_string("./input.txt");

    if input.is_err() {
        return Err("Error reading input file");
    }

    Ok("Not implemented yet")
}
