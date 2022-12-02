use std::fs;

pub fn run() {
    println!("## DAY 1");
    let content = fs::read_to_string("input-day-1");
    if content.is_err() {
        eprintln!("{}", content.unwrap_err());
        return;
    }

    let mut calories: Vec<i32> = content
        .unwrap()
        .split("\n\n")
        .map(|group| {
            let sum = group
                .split("\n")
                .fold(0, |sum, line| sum + line.parse::<i32>().unwrap_or(0));

            sum
        })
        .collect();

    // let mut calories = vec![0];
    // for line in content.unwrap().lines() {
    //     if line.is_empty() {
    //         calories.insert(0, 0);
    //     } else {
    //         calories[0] += i32::from(line.parse().unwrap_or(0));
    //     }
    // }

    calories.sort();
    calories.reverse();

    let top_three: Vec<i32> = calories.into_iter().take(3).collect();

    println!("\tTop 3: {:?}", top_three);
    println!("\tTotal: {}", top_three.iter().sum::<i32>());
    println!("\n");
}
