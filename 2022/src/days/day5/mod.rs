use std::fs::read_to_string;

#[derive(Debug)]
#[allow(dead_code)]
struct Instruction {
    move_number: usize,
    from: usize,
    to: usize,
}

static STACK_WIDTH: usize = 3;

pub fn solve() -> &'static str {
    let input = read_to_string("./src/days/day5/input.txt").unwrap();
    let raw_input: Vec<&str> = input.split("\n").filter(|x| !x.is_empty()).collect();
    let mut stacks = read_stacks(&raw_input).unwrap();
    let instructions = read_instructions(&raw_input).unwrap();

    println!("Starting Stacks: {:?}", stacks);

    // let result_9000 = execute_inscructions_9000(&mut stacks, instructions);
    let result_9001 = execute_inscructions_9001(&mut stacks, instructions);

    if result_9001.is_err() {
        return "Error";
    }

    println!("Finishing Stacks: {:?}", stacks);

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

    stacks.iter_mut().for_each(|x| x.reverse());

    Ok(stacks)
}

fn read_instructions<'b>(input: &'b Vec<&'b str>) -> Result<Vec<Instruction>, &str> {
    let mut instructions = Vec::new();

    for line in input {
        if line.chars().nth(0).unwrap() != 'm' {
            continue;
        }

        let move_number = line.split(" ").nth(1).unwrap().parse::<usize>().unwrap();
        let from = line.split(" ").nth(3).unwrap().parse::<usize>().unwrap();
        let to = line.split(" ").nth(5).unwrap().parse::<usize>().unwrap();

        instructions.push(Instruction{
            move_number,
            from: from - 1,
            to: to - 1,
        });
    }

    Ok(instructions)
}

fn _execute_inscructions_9000(stacks: &mut Vec<Vec<char>>, instructions: Vec<Instruction>) -> Result<&mut Vec<Vec<char>>, &str> {
    for instruction in instructions {
        for _i in 0..instruction.move_number {
            let from_char = stacks[instruction.from].pop();

            if from_char.is_none() {
                return Err("Something went wrong :*(");
            }

            stacks[instruction.to].push(from_char.unwrap());
        }
    }

    Ok(stacks)
}

fn execute_inscructions_9001(stacks: &mut Vec<Vec<char>>, instructions: Vec<Instruction>) -> Result<&mut Vec<Vec<char>>, &str> {
    for instruction in instructions {
        let mut from_chars = Vec::new();

        for _i in 0..instruction.move_number {
            let from_char = stacks[instruction.from].pop();

            if from_char.is_none() {
                return Err("Something went wrong :*(");
            }

            from_chars.push(from_char.unwrap());
        }

        from_chars.reverse();
        stacks[instruction.to].append(&mut from_chars);
    }

    Ok(stacks)
}
