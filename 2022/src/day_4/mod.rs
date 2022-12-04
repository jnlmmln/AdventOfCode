use std::{fs, ops::RangeInclusive};

pub fn run() {
    println!("## DAY 4");

    part_1();
    part_2();

    println!("\n");
}

fn part_2() {
    println!("### Part 2");
    let content = fs::read_to_string("input-day-4");
    if content.is_err() {
        eprintln!("{}", content.unwrap_err());
        return;
    }

    let total = content.unwrap().lines().fold(0, |total, line| {
        let pair: Vec<&str> = line.split(',').collect();

        if pair.len() != 2 {
            return total + 0;
        }

        let range1 = get_range(pair.first().unwrap());
        let range2 = get_range(pair.last().unwrap());

        if range1.contains_partly(&range2) || range2.contains_partly(&range1) {
            return total + 1;
        }

        total
    });

    println!("\tTotal: {total}");
}

fn part_1() {
    println!("### Part 1");
    let content = fs::read_to_string("input-day-4");
    if content.is_err() {
        eprintln!("{}", content.unwrap_err());
        return;
    }

    let total = content.unwrap().lines().fold(0, |total, line| {
        let pair: Vec<&str> = line.split(',').collect();

        if pair.len() != 2 {
            return total + 0;
        }

        let range1 = get_range(pair.first().unwrap());
        let range2 = get_range(pair.last().unwrap());

        if range1.contains_fully(&range2) || range2.contains_fully(&range1) {
            return total + 1;
        }

        total
    });

    println!("\tTotal: {total}");
}

fn get_range(input: &str) -> RangeInclusive<usize> {
    let splitted: Vec<&str> = input.split('-').collect();

    let start = usize::from_str_radix(splitted.first().unwrap(), 10).unwrap();
    let end = usize::from_str_radix(splitted.last().unwrap(), 10).unwrap();

    RangeInclusive::new(start, end)
}

trait RangeIncludesRange {
    fn contains_fully(self: &Self, other: &Self) -> bool;
    fn contains_partly(self: &Self, other: &Self) -> bool;
}
impl RangeIncludesRange for RangeInclusive<usize> {
    fn contains_fully(self: &Self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }
    fn contains_partly(self: &Self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end())
    }
}
