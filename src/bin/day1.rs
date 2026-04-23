use std::fs::read_to_string;

fn day1(input: &str) {
    let mut groups: Vec<i32> = Vec::new();
    let mut max_calories: i32 = 0;
    let mut calories: i32 = 0;

    for line in input.lines() {
        if line.is_empty() {
            groups.push(calories);
            max_calories = max_calories.max(calories);
            calories = 0;
            continue;
        }

        let temp: i32 = line.parse().expect("not a number");
        calories += temp;
    }

    // last group
    groups.push(calories);
    max_calories = max_calories.max(calories);

    groups.sort_unstable_by(|a, b| b.cmp(a));

    let top3_sum: i32 = groups.iter().take(3).sum();

    println!("part1: {}", max_calories);
    println!("part2: {}", top3_sum);
}

fn main() {
    let input = read_to_string("src/input/day1.input").unwrap();

    day1(&input);
}
