use std::fs::read_to_string;

pub fn solve() -> &'static str {
    let input = read_to_string("./src/days/day4/input.txt").unwrap();
    let pairs: Vec<&str> = input.split("\n").filter(|x| !x.is_empty()).collect();

    let mut total_overlapping = 0;
    let mut total_overlapping_at_all = 0;

    for pair in &pairs {
        let first_elf = pair.split(",").nth(0).unwrap();
        let second_elf = pair.split(",").nth(1).unwrap();

        let first_elf_start = first_elf.split("-").nth(0).unwrap().parse::<i32>().unwrap();
        let first_elf_end = first_elf.split("-").nth(1).unwrap().parse::<i32>().unwrap();
        let second_elf_start = second_elf.split("-").nth(0).unwrap().parse::<i32>().unwrap();
        let second_elf_end = second_elf.split("-").nth(1).unwrap().parse::<i32>().unwrap();

        if is_completely_overlapping(first_elf_start, first_elf_end, second_elf_start, second_elf_end) {
            total_overlapping += 1;
            total_overlapping_at_all += 1;

            continue;
        }

        if is_overlapping_at_all(first_elf_start, first_elf_end, second_elf_start, second_elf_end) {
            total_overlapping_at_all += 1;
        }
    }

    println!("Total overlapping: {:?}", total_overlapping);
    println!("Total overlapping at all: {:?}", total_overlapping_at_all);

    "Day four solution"
}

fn is_completely_overlapping(first_elf_start: i32, first_elf_end: i32, second_elf_start: i32, second_elf_end: i32) -> bool {
    if first_elf_start >= second_elf_start && first_elf_end <= second_elf_end {
        return true;
    }

    if second_elf_start >= first_elf_start && second_elf_end <= first_elf_end {
        return true;
    }

    false
}

fn is_overlapping_at_all(first_elf_start: i32, first_elf_end: i32, second_elf_start: i32, second_elf_end: i32) -> bool {
    if first_elf_start >= second_elf_start && first_elf_start <= second_elf_end {
        return true;
    }

    if second_elf_start >= first_elf_start && second_elf_start <= first_elf_end {
        return true;
    }

    false
}
