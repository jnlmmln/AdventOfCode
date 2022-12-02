use std::fs;

enum RockPaperScissors {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
    Lose,
    Draw,
    Win,
}

pub fn run() {
    println!("## DAY 2");

    part_1();
    part_2();

    println!("\n");
}

fn part_2() {
    println!("### Part 2");
    let content = fs::read_to_string("input-day-2");
    if content.is_err() {
        eprintln!("{}", content.unwrap_err());
        return;
    }

    let total = content.unwrap().lines().fold(0, |sum, game| {
        let choices: Vec<RockPaperScissors> = game
            .split_whitespace()
            .map(|input| match input {
                "A" => RockPaperScissors::Rock,
                "B" => RockPaperScissors::Paper,
                "C" => RockPaperScissors::Scissors,
                "X" => RockPaperScissors::Lose,
                "Y" => RockPaperScissors::Draw,
                "Z" => RockPaperScissors::Win,
                _ => unimplemented!(),
            })
            .collect();

        if choices.len() != 2 {
            return sum + 0;
        }

        let opponent = choices.first().unwrap();
        let i_should = choices.last().unwrap();
        sum + match opponent {
            RockPaperScissors::Rock => match i_should {
                RockPaperScissors::Win => 6 + RockPaperScissors::Paper as i32,
                RockPaperScissors::Draw => 3 + RockPaperScissors::Rock as i32,
                RockPaperScissors::Lose => 0 + RockPaperScissors::Scissors as i32,
                _ => unimplemented!(),
            },
            RockPaperScissors::Paper => match i_should {
                RockPaperScissors::Win => 6 + RockPaperScissors::Scissors as i32,
                RockPaperScissors::Draw => 3 + RockPaperScissors::Paper as i32,
                RockPaperScissors::Lose => 0 + RockPaperScissors::Rock as i32,
                _ => unimplemented!(),
            },
            RockPaperScissors::Scissors => match i_should {
                RockPaperScissors::Win => 6 + RockPaperScissors::Rock as i32,
                RockPaperScissors::Draw => 3 + RockPaperScissors::Scissors as i32,
                RockPaperScissors::Lose => 0 + RockPaperScissors::Paper as i32,
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    });

    println!("\tTotal: {total}");
}

fn part_1() {
    println!("### Part 1");
    let content = fs::read_to_string("input-day-2");
    if content.is_err() {
        eprintln!("{}", content.unwrap_err());
        return;
    }

    let total = content.unwrap().lines().fold(0, |total, game| {
        let choices: Vec<RockPaperScissors> = game
            .split_whitespace()
            .map(|input| {
                if input == "A" || input == "X" {
                    return RockPaperScissors::Rock;
                } else if input == "B" || input == "Y" {
                    return RockPaperScissors::Paper;
                } else {
                    return RockPaperScissors::Scissors;
                }
            })
            .collect();

        if choices.len() != 2 {
            return 0;
        }

        let opponent = choices.first().unwrap();
        let my = choices.last().unwrap();

        total
            + match my {
                RockPaperScissors::Rock => {
                    1 + match opponent {
                        RockPaperScissors::Scissors => 6,
                        RockPaperScissors::Rock => 3,
                        RockPaperScissors::Paper => 0,
                        _ => unimplemented!(),
                    }
                }
                RockPaperScissors::Paper => {
                    2 + match opponent {
                        RockPaperScissors::Rock => 6,
                        RockPaperScissors::Paper => 3,
                        RockPaperScissors::Scissors => 0,
                        _ => unimplemented!(),
                    }
                }
                RockPaperScissors::Scissors => {
                    3 + match opponent {
                        RockPaperScissors::Paper => 6,
                        RockPaperScissors::Scissors => 3,
                        RockPaperScissors::Rock => 0,
                        _ => unimplemented!(),
                    }
                }
                _ => unimplemented!(),
            }
    });

    println!("\tTotal: {total}");
}
