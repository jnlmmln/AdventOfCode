use clap::{ArgGroup, Parser};

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

#[derive(Parser)]
#[command(group(
            ArgGroup::new("mode")
                .required(true)
                .args(["day", "all"]),
        ))]
struct Cli {
    /// The day to run
    #[arg(short, long, value_name = "1-24", value_parser = clap::value_parser!(u8).range(1..25))]
    day: Option<u8>,

    /// Run all days
    #[arg(short, long)]
    all: bool,
}

fn main() {
    let args = Cli::parse();

    println!("\n# AoC 2022");

    if args.all {
        day_1::run();
        day_2::run();
        day_3::run();
        day_4::run();
        day_5::run();
    } else {
        if let Some(day) = args.day {
            match day {
                1 => day_1::run(),
                2 => day_2::run(),
                3 => day_3::run(),
                4 => day_4::run(),
                5 => day_5::run(),
                _ => println!("Day {day} is not implemented"),
            }
        }
    }
}
