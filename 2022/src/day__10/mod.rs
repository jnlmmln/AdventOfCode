use std::fs;

pub fn run() {
    println!("## DAY 10");

    part_1();
    part_2();

    println!("\n");
}

fn part_2() {
    let content = fs::read_to_string("input-day_10");
    if content.is_err() {
        eprintln!("{}", content.unwrap_err());
        return;
    }

    let content = content.unwrap();
    let mut reg_x = 1;
    let mut cycle = 0;
    let mut crt: Vec<String> = vec![];

    for line in content.lines() {
        let (cmd, param) = line.split_once(" ").unwrap_or(("noop", ""));

        match cmd {
            "noop" => {
                if (cycle % 40) >= reg_x - 1 && (cycle % 40) <= reg_x + 1 {
                    crt.push(String::from("#"));
                } else {
                    crt.push(String::from("."));
                }

                cycle += 1;
            }
            "addx" => {
                for _ in 0..2 {
                    if (cycle % 40) >= reg_x - 1 && (cycle % 40) <= reg_x + 1 {
                        crt.push(String::from("#"));
                    } else {
                        crt.push(String::from("."));
                    }

                    cycle += 1;
                }

                reg_x += param.parse::<i32>().unwrap();
            }
            _ => {}
        }
    }

    println!("### Part 2");
    print!("\tResult:");
    print_crt(&crt);
}

fn part_1() {
    let content = fs::read_to_string("input-day_10");
    if content.is_err() {
        eprintln!("{}", content.unwrap_err());
        return;
    }

    let content = content.unwrap();
    let mut reg_x = 1;
    let mut cycle = 0;
    let mut result = 0;

    for line in content.lines() {
        let (cmd, param) = line.split_once(" ").unwrap_or(("noop", ""));
        // println!("{} {}", cmd, param);
        // println!("Cycle\tReg X");
        // println!("{}\t{}", cycle, reg_x);

        match cmd {
            "noop" => {
                cycle += 1;
                if (cycle - 20) % 40 == 0 {
                    println!("Cycle {} * {} = {}", cycle, reg_x, cycle * reg_x);
                    result += cycle * reg_x;
                }
            }
            "addx" => {
                for _ in 0..2 {
                    cycle += 1;
                    if (cycle - 20) % 40 == 0 {
                        println!("Cycle {} * {} = {}", cycle, reg_x, cycle * reg_x);
                        result += cycle * reg_x;
                    }
                }

                reg_x += param.parse::<i32>().unwrap();
            }
            _ => {}
        }
    }

    println!("### Part 1");
    println!("\tResult: {:?}", result);
}

fn print_crt(crt: &Vec<String>) {
    // print!("{}", termion::clear::All);
    for x in crt.iter().enumerate() {
        if x.0 % 40 == 0 {
            println!();
        }
        print!("{}", x.1);
    }
    println!();

    // std::thread::sleep(std::time::Duration::from_millis(100));
}
