use std::fs::read_to_string;

const WIN: i32 = 6;
const LOSE: i32 = 0;
const DRAW: i32 = 3;

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;

fn part1(input: &str) {
    let mut score = 0;

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let (opp, me) = (parts.next().unwrap(), parts.next().unwrap());

        match opp {
            "A" => match me {
                "X" => score += DRAW + ROCK,
                "Y" => score += WIN + PAPER,
                "Z" => score += LOSE + SCISSORS,
                _ => continue,
            },

            "B" => match me {
                "X" => score += LOSE + ROCK,
                "Y" => score += DRAW + PAPER,
                "Z" => score += WIN + SCISSORS,
                _ => continue,
            },

            "C" => match me {
                "X" => score += WIN + ROCK,
                "Y" => score += LOSE + PAPER,
                "Z" => score += DRAW + SCISSORS,
                _ => continue,
            },

            _ => continue,
        }
    }

    println!("part1 score: {}", score);
}

fn part2(input: &str) {
    let mut score = 0;

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let (opp, me) = (parts.next().unwrap(), parts.next().unwrap());

        match opp {
            "A" => match me {
                "X" => score += LOSE + SCISSORS,
                "Y" => score += DRAW + ROCK,
                "Z" => score += WIN + PAPER,
                _ => continue,
            },

            "B" => match me {
                "X" => score += LOSE + ROCK,
                "Y" => score += DRAW + PAPER,
                "Z" => score += WIN + SCISSORS,
                _ => continue,
            },

            "C" => match me {
                "X" => score += LOSE + PAPER,
                "Y" => score += DRAW + SCISSORS,
                "Z" => score += WIN + ROCK,
                _ => continue,
            },

            _ => continue,
        }
    }

    println!("part2 score: {}", score);
}

fn main() {
    let input = read_to_string("src/input/day2.input").unwrap();

    part1(&input);
    part2(&input);
}
