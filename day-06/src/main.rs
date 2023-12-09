use regex::Regex;
use std::fs;

fn main() {
    let content = fs::read_to_string("input").unwrap();
    let number_matcher = Regex::new(r"(?<n>\d+)").unwrap();
    let times: Vec<usize> = number_matcher
        .captures_iter(
            content
                .lines()
                .nth(0)
                .unwrap()
                .split("Time: ")
                .nth(1)
                .unwrap(),
        )
        .map(|c| c["n"].parse::<usize>().unwrap())
        .collect();
    let distances: Vec<usize> = number_matcher
        .captures_iter(
            content
                .lines()
                .nth(1)
                .unwrap()
                .split("Distance: ")
                .nth(1)
                .unwrap(),
        )
        .map(|c| c["n"].parse::<usize>().unwrap())
        .collect();
    let mut part1 = 0;
    for (i, t) in times.iter().enumerate() {
        let distance = distances[i];
        let mut wins = 0;
        for x in 0..*t {
            if x * (t - x) > distance {
                wins += 1;
            }
        }
        if part1 == 0{
            part1 = wins;
        } else {
            part1 = part1 * wins;
        }
    }

    println!("Part 1 {part1}")
}
