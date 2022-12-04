use std::fs;

pub fn run() {
    println!("## DAY 3");

    part_1();
    part_2();

    println!("\n");
}

fn part_2() {
    println!("### Part 2");
    let content = fs::read_to_string("input-day-3");
    if content.is_err() {
        eprintln!("{}", content.unwrap_err());
        return;
    }

    let input = content.unwrap();

    let total = input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .fold(0, |mut total, rucksacks| {
            'outer: for r0 in rucksacks[0].chars() {
                for r1 in rucksacks[1].chars() {
                    if r0 == r1 {
                        for r2 in rucksacks[2].chars() {
                            if r0 == r2 {
                                let priority = r0 as i32 - if r0.is_lowercase() { 96 } else { 38 };
                                total += priority;
                                break 'outer;
                            }
                        }
                    }
                }
            }

            total
        });

    println!("\tTotal: {total}");
}

fn part_1() {
    println!("### Part 1");
    let content = fs::read_to_string("input-day-3");
    if content.is_err() {
        eprintln!("{}", content.unwrap_err());
        return;
    }

    let total = content.unwrap().lines().fold(0, |mut total, rucksack| {
        let compartments = rucksack.split_at(rucksack.len() / 2);
        let mut matches = vec![];

        for c0 in compartments.0.chars() {
            for c1 in compartments.1.chars() {
                if c0 == c1 && matches.contains(&c0) == false {
                    let priority = c0 as i32 - if c0.is_lowercase() { 96 } else { 38 };
                    total += priority;
                    matches.push(c0);
                }
            }
        }
        total
    });

    println!("\tTotal: {total}");
}
