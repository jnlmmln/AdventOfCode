use std::{fs, thread::sleep};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Position(i32, i32);

pub fn run() {
    println!("## DAY 9");

    part_1();
    // part_2();

    println!("\n");
}

fn part_1() {
    let content = fs::read_to_string("input-day-9");
    if content.is_err() {
        eprintln!("{}", content.unwrap_err());
        return;
    }

    let content = content.unwrap();
    let mut rope: Vec<Position> = vec![Position(0, 0); 10];
    let mut visited_positions: Vec<Position> = vec![rope.last().unwrap().clone()];

    for line in content.lines() {
        let (direction, count) = line.split_once(" ").unwrap();

        // println!("{} - {}", direction, count);

        for _ in 0..count.parse::<u32>().unwrap() {
            rope = rope.iter().fold(vec![], |mut acc, cur| {
                let mut new = cur.clone();
                if acc.is_empty() {
                    match direction {
                        "U" => new = Position(cur.0, cur.1 + 1),
                        "R" => new = Position(cur.0 + 1, cur.1),
                        "D" => new = Position(cur.0, cur.1 - 1),
                        "L" => new = Position(cur.0 - 1, cur.1),
                        _ => (),
                    }
                    // println!("Head {:?}", new);

                    acc.push(new);
                    return acc;
                }

                let pre = acc.last().unwrap();

                // X
                if pre.0 > new.0 + 1 {
                    new = Position(new.0 + 1, new.1);

                    if pre.1 > new.1 {
                        new = Position(new.0, new.1 + 1);
                    }
                    if pre.1 < new.1 {
                        new = Position(new.0, new.1 - 1);
                    }
                }
                if pre.0 < new.0 - 1 {
                    new = Position(new.0 - 1, new.1);

                    if pre.1 > new.1 {
                        new = Position(new.0, new.1 + 1);
                    }
                    if pre.1 < new.1 {
                        new = Position(new.0, new.1 - 1);
                    }
                }

                // Y
                if pre.1 > new.1 + 1 {
                    new = Position(new.0, new.1 + 1);

                    if pre.0 > new.0 {
                        new = Position(new.0 + 1, new.1);
                    }
                    if pre.0 < new.0 {
                        new = Position(new.0 - 1, new.1);
                    }
                }
                if pre.1 < new.1 - 1 {
                    new = Position(new.0, new.1 - 1);

                    if pre.0 > new.0 {
                        new = Position(new.0 + 1, new.1);
                    }
                    if pre.0 < new.0 {
                        new = Position(new.0 - 1, new.1);
                    }
                }
                // println!("New {:?}", new);
                acc.push(new);
                return acc;
            });

            let last = rope.last().unwrap();

            if visited_positions
                .iter()
                .any(|x| x.0 == last.0 && x.1 == last.1)
                == false
            {
                visited_positions.push(last.clone());
            }
        }
    }

    // print_grid(&rope, &visited_positions);

    println!("### Part 2");
    println!("\tResult: {:?}", visited_positions.len());
}

#[allow(dead_code)]
fn print_grid(positions: &Vec<Position>, visited_positions: &Vec<Position>) {
    print!("{}", termion::clear::All);
    // println!("{:?}", positions);

    let max_x = positions.iter().max_by_key(|x| x.0).unwrap().0;
    let max_y = positions.iter().max_by_key(|x| x.1).unwrap().1;

    let min_x = positions.iter().min_by_key(|x| x.0).unwrap().0;
    let min_y = positions.iter().min_by_key(|x| x.1).unwrap().1;

    let max_visited_x = visited_positions.iter().max_by_key(|x| x.0).unwrap().0;
    let max_visited_y = visited_positions.iter().max_by_key(|x| x.1).unwrap().1;

    let min_visited_x = visited_positions.iter().min_by_key(|x| x.0).unwrap().0;
    let min_visited_y = visited_positions.iter().min_by_key(|x| x.1).unwrap().1;

    let max_x = max_x.max(max_visited_x).max(10);
    let min_x = min_x.min(min_visited_x).min(-10);

    let max_y = max_y.max(max_visited_y).max(10);
    let min_y = min_y.min(min_visited_y).min(-10);

    for y in (min_y..max_y).rev() {
        for x in min_x..max_x {
            let mut sign = String::from(".");

            for pos in visited_positions.iter().enumerate() {
                if pos.1 == &Position(x, y) {
                    sign = "#".to_string();
                }
            }

            for pos in positions.iter().enumerate() {
                if pos.1 == &Position(x, y) {
                    sign = pos.0.to_string()
                }
            }
            print!("{}", sign);
        }
        println!("");
    }

    // let stdin = stdin();
    // let mut stdin = stdin.lock();
    // stdin.read_line();

    sleep(std::time::Duration::from_millis(20));
}
