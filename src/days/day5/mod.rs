use std::fs::read_to_string;

#[derive(Debug)]
struct Instruction {
    move_number: usize,
    from: usize,
    to: usize,
}

static STACK_WIDTH: usize = 3;

pub fn solve() -> &'static str {
    let input = read_to_string("./src/days/day5/input.txt").unwrap();
    let raw_input: Vec<&str> = input.split("\n").filter(|x| !x.is_empty()).collect();
    let _stacks = read_stacks(&raw_input).unwrap();
    let _instructions = read_instructions(&raw_input).unwrap();

    "Day five solution"
}

fn read_stacks<'a>(input: &'a Vec<&'a str>) -> Result<Vec<Vec<char>>, &str> {
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

    Ok(stacks)
}

fn read_instructions<'b>(input: &'b Vec<&'b str>) -> Result<Vec<Instruction>, &str> {
    let mut instructions = Vec::new();

    for line in input {
        let move_number = line.split(" ").nth(1).unwrap().parse::<usize>().unwrap();
        let from = line.split(" ").nth(3).unwrap().parse::<usize>().unwrap();
        let to = line.split(" ").nth(5).unwrap().parse::<usize>().unwrap();

        println!("{:?}", move_number);

        instructions.push(Instruction{
            move_number,
            from,
            to,
        });
    }

    Ok(instructions)
}
