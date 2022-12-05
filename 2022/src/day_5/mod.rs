use regex::Regex;
use std::fs;

pub fn run() {
    println!("## DAY 5");

    part_1();
    part_2();

    println!("\n");
}

fn part_2() {
    println!("### Part 2");
    let content = fs::read_to_string("input-day-5");
    if content.is_err() {
        eprintln!("{}", content.unwrap_err());
        return;
    }

    let content = content.unwrap();
    let (init, instructions) = content.split_once("\n\n").unwrap_or(("", ""));

    let mut stacks = init.lines().rev().skip(1).fold(vec![], |mut stacks, line| {
        let vec: Vec<char> = line.chars().collect();

        vec.chunks(4).enumerate().for_each(|(id, column)| {
            if stacks.len() <= id {
                stacks.push(vec![]);
            }
            if column[1] != ' ' {
                stacks[id].push(column[1]);
            }
        });

        stacks
    });

    let re = Regex::new(r"move (\d*) from (\d*) to (\d*)").unwrap();

    instructions.lines().for_each(|instruction| {
        let cap = re.captures(instruction).unwrap();

        let count = usize::from_str_radix(&cap[1], 10).unwrap();
        let from = usize::from_str_radix(&cap[2], 10).unwrap() - 1;
        let to = usize::from_str_radix(&cap[3], 10).unwrap() - 1;
        let stack_size = stacks[from].len();

        if stack_size >= count {
            let mut v: Vec<char> = stacks[from].drain(stack_size - count..).collect();
            stacks[to].append(&mut v);
        }
    });

    println!(
        "\tResult: {}",
        stacks.iter().fold(String::new(), |result, stack| {
            result + &stack.last().unwrap_or(&' ').to_string()
        })
    );
}

fn part_1() {
    println!("### Part 1");
    let content = fs::read_to_string("input-day-5");
    if content.is_err() {
        eprintln!("{}", content.unwrap_err());
        return;
    }

    let content = content.unwrap();
    let (init, instructions) = content.split_once("\n\n").unwrap_or(("", ""));

    let mut stacks = init.lines().rev().skip(1).fold(vec![], |mut stacks, line| {
        let vec: Vec<char> = line.chars().collect();

        vec.chunks(4).enumerate().for_each(|(id, column)| {
            if stacks.len() <= id {
                stacks.push(vec![]);
            }
            if column[1] != ' ' {
                stacks[id].push(column[1]);
            }
        });

        stacks
    });

    let re = Regex::new(r"move (\d*) from (\d*) to (\d*)").unwrap();

    instructions.lines().for_each(|instruction| {
        let cap = re.captures(instruction).unwrap();

        let count = usize::from_str_radix(&cap[1], 10).unwrap();
        let from = usize::from_str_radix(&cap[2], 10).unwrap() - 1;
        let to = usize::from_str_radix(&cap[3], 10).unwrap() - 1;

        for _ in 0..count {
            if stacks[from].len() > 0 {
                let v = stacks[from].pop().unwrap();
                stacks[to].push(v);
            }
        }
    });

    println!(
        "\tResult: {}",
        stacks.iter().fold(String::new(), |result, stack| {
            result + &stack.last().unwrap_or(&' ').to_string()
        })
    );
}
