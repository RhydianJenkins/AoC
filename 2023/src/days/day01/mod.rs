use std::fs::read_to_string;

pub fn solve() -> Result<&'static str, &'static str> {
    let input = read_to_string("./src/days/day01/input.txt");

    if input.is_err() {
        return Err("Error reading input file");
    }

    Ok("Not implemented yet")
}

pub mod tests {
    use super::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve(), Ok("Not implemented yet"));
    }
}
