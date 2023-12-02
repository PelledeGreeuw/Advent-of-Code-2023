use regex::Regex;
use std::fs;

fn main() {
    let content = fs::read_to_string("input").unwrap();
    let red = 12;
    let green = 13;
    let blue = 14;
    let mut total = 0;
    let matcher = Regex::new(r"Game (?<game>\d+): (?<states>.+)").unwrap();
    let state_matcher = Regex::new(r"((?:\d+ \w+,? ?)+;? ?)").unwrap();
    let color_matcher = Regex::new(r"(?<amount>\d+) (?<color>\w+)").unwrap();
    for line in content.lines() {
        let game = matcher.captures(line).unwrap();
        let states = state_matcher
            .captures_iter(&game["states"])
            .map(|c| c.extract());
        let mut impossible = false;
        'states: for (_, [state]) in states {
            for (_, [amount, color]) in color_matcher.captures_iter(&state).map(|c| c.extract()) {
                if color == "red" && amount.parse::<u32>().unwrap() > red {
                    impossible = true;
                    break 'states;
                } else if color == "green" && amount.parse::<u32>().unwrap() > green {
                    impossible = true;
                    break 'states;
                } else if color == "blue" && amount.parse::<u32>().unwrap() > blue {
                    impossible = true;
                    break 'states;
                }
            }
        }
        if !impossible {
            total += game["game"].parse::<u32>().unwrap();
        }
    }

    let mut part_2_total = 0;

    for line in content.lines() {
        let game = matcher.captures(line).unwrap();
        let states = state_matcher
            .captures_iter(&game["states"])
            .map(|c| c.extract());
        let mut min_red = 0;
        let mut min_blue = 0;
        let mut min_green = 0;
        for (_, [state]) in states {
            for (_, [amount, color]) in color_matcher.captures_iter(&state).map(|c| c.extract()) {
                let int_amount = amount.parse::<u32>().unwrap();
                if color == "red" && min_red < int_amount {
                    min_red = int_amount;
                } else if color == "green" && min_green < int_amount {
                    min_green = int_amount;
                } else if color == "blue" && min_blue < int_amount {
                    min_blue = int_amount;
                }
            }
        }
        println!("{min_blue}, {min_red}, {min_green}");
        part_2_total += min_blue * min_red * min_green;
    }
    println!("{total}");
    println!("{part_2_total}");
}
