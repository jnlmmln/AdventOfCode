use std::fs;

pub fn run() {
    println!("## DAY 8");

    part_1();
    part_2();

    println!("\n");
}

fn part_2() {
    let content = fs::read_to_string("input-day-8");
    if content.is_err() {
        eprintln!("{}", content.unwrap_err());
        return;
    }

    let content = content.unwrap();
    let mut grid: Vec<Vec<char>> = vec![];

    for line in content.lines() {
        grid.push(line.chars().collect::<Vec<char>>());
    }

    let mut best_view_score = 0;
    let x_axis = grid[0].len();
    let y_axis = grid.len();

    for i in 0..y_axis {
        for j in 0..x_axis {
            let mut visible_to_top = 0;
            let mut visible_to_right = 0;
            let mut visible_to_bottom = 0;
            let mut visible_to_left = 0;

            // To top
            for x in (0..i).rev() {
                visible_to_top += 1;
                if grid[x][j] >= grid[i][j] {
                    break;
                }
            }
            // To right
            for x in (j + 1)..x_axis {
                visible_to_right += 1;
                if grid[i][x] >= grid[i][j] {
                    break;
                }
            }
            // To bottom
            for x in (i + 1)..y_axis {
                visible_to_bottom += 1;
                if grid[x][j] >= grid[i][j] {
                    break;
                }
            }
            // To left
            for x in (0..j).rev() {
                visible_to_left += 1;
                if grid[i][x] >= grid[i][j] {
                    break;
                }
            }

            let result = visible_to_top * visible_to_right * visible_to_bottom * visible_to_left;
            if result > best_view_score {
                best_view_score = result;
            }
        }
    }

    println!("### Part 2");
    println!("\tResult: {}", best_view_score);
}

fn part_1() {
    let content = fs::read_to_string("input-day-8");
    if content.is_err() {
        eprintln!("{}", content.unwrap_err());
        return;
    }

    let content = content.unwrap();
    let mut grid: Vec<Vec<char>> = vec![];

    for line in content.lines() {
        grid.push(line.chars().collect::<Vec<char>>());
    }

    let mut visible_trees = 0;
    let x_axis = grid[0].len();
    let y_axis = grid.len();

    for i in 0..y_axis {
        for j in 0..x_axis {
            if i == 0 || i == y_axis - 1 {
                visible_trees += 1;
                continue;
            }
            if j == 0 || j == x_axis - 1 {
                visible_trees += 1;
                continue;
            }

            let mut visible_from_top = true;
            let mut visible_from_right = true;
            let mut visible_from_bottom = true;
            let mut visible_from_left = true;

            // From top
            for x in 0..i {
                if grid[x][j] >= grid[i][j] {
                    visible_from_top = false;
                }
            }
            // From right
            for x in (j + 1)..x_axis {
                if grid[i][x] >= grid[i][j] {
                    visible_from_right = false;
                }
            }
            // From bottom
            for x in (i + 1)..y_axis {
                if grid[x][j] >= grid[i][j] {
                    visible_from_bottom = false;
                }
            }
            // From left
            for x in 0..j {
                if grid[i][x] >= grid[i][j] {
                    visible_from_left = false;
                }
            }

            if visible_from_top || visible_from_right || visible_from_bottom || visible_from_left {
                visible_trees += 1;
            }
        }
    }

    println!("### Part 1");
    println!("\tResult: {}", visible_trees);
}
