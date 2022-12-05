use std::fs::read_to_string;

static STACK_WIDTH: usize = 3;

pub fn solve() -> &'static str {
    let input = read_to_string("./src/days/day5/input.txt").unwrap();
    let raw_input: Vec<&str> = input.split("\n").filter(|x| !x.is_empty()).collect();
    let stacks = read_stacks(raw_input).unwrap();

    println!("Day 5, part 1: {:?}", stacks);

    "Day five solution"
}

fn read_stacks(input: Vec<&str>) -> Result<Vec<Vec<char>>, &str> {
    let mut stacks: Vec<Vec<char>> = Vec::new();

    let num_cols = input.first().unwrap().chars().count() / STACK_WIDTH - (STACK_WIDTH - 1);
    for _i in 0..num_cols {
        stacks.push(Vec::new());
    }

    for line in input {
        if line.contains("1") {
            break;
        }

        for i in 0..num_cols {
            let index_to_check = (i * STACK_WIDTH) + (i + 1) as usize;
            let next_char = line.chars().nth(index_to_check);
            let stack = &mut stacks[i];

            if next_char.unwrap() != ' ' {
                stack.push(next_char.unwrap());
            }
        }
    }

    return Ok(stacks);
}
