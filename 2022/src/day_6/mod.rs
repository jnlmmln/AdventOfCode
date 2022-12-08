use std::fs;

pub fn run() {
    println!("## DAY 6");

    part_1();
    part_2();

    println!("\n");
}

fn part_2() {
    println!("### Part 2");
    let content = fs::read_to_string("input-day-6");
    if content.is_err() {
        eprintln!("{}", content.unwrap_err());
        return;
    }

    let content = content.unwrap();
    let content: Vec<char> = content.chars().collect();

    let mark = content
        .windows(14)
        .enumerate()
        .find(|(_, chars)| is_unique(chars.to_vec()));

    println!("\tResult: {}", mark.unwrap().0 + 14);
}

fn part_1() {
    println!("### Part 1");
    let content = fs::read_to_string("input-day-6");
    if content.is_err() {
        eprintln!("{}", content.unwrap_err());
        return;
    }

    let content = content.unwrap();
    let content: Vec<char> = content.chars().collect();

    let mark = content
        .windows(4)
        .enumerate()
        .find(|(_, chars)| is_unique(chars.to_vec()));

    println!("\tResult: {}", mark.unwrap().0 + 4);
}

fn is_unique(chars: Vec<char>) -> bool {
    let mut seen_chars = Vec::new();
    for c in chars {
        if seen_chars.contains(&c) {
            return false;
        }
        seen_chars.push(c);
    }
    true
}
