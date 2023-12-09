use num::integer;
use regex::Regex;
use std::{collections::HashMap, fs};

fn main() {
    let content = fs::read_to_string("input").unwrap();
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    let rl: Vec<String> = content
        .lines()
        .nth(0)
        .unwrap()
        .chars()
        .map(|c| c.to_string())
        .collect();
    let mut start: Vec<&str> = Vec::new();
    let matcher = Regex::new(r"(?<src>\w+) = \((?<r>\w+), (?<l>\w+)\)").unwrap();
    for (_, [src, l, r]) in matcher.captures_iter(&content).map(|c| c.extract()) {
        let mut v: Vec<&str> = Vec::new();
        v.push(l);
        v.push(r);
        if src.ends_with("A") {
            start.push(src);
        }
        map.insert(src, v);
    }
    let mut part1 = 0;
    let mut i = 0;
    let mut src = "AAA";
    while src.ends_with("Z") {
        if rl[i] == "R" {
            src = map.get(src).unwrap()[1];
        } else {
            src = map.get(src).unwrap()[0];
        }
        i += 1;
        part1 += 1;
        if i == rl.len() {
            i = 0;
        }
    }
    println!("Part 1 {part1}");

    let mut values: Vec<usize> = Vec::new();
    for val in start {
        let mut part2 = 0;
        i = 0;
        let mut source = val;
        while !source.ends_with("Z") {
            if rl[i] == "R" {
                source = map.get(source).unwrap()[1];
            } else {
                source = map.get(source).unwrap()[0];
            }
            i += 1;
            part2 += 1;
            if i == rl.len() {
                i = 0;
            }
        }
        values.push(part2);
    }
    let mut part2 = integer::lcm(values[0], values[1]);
    for (i, value) in values.iter().enumerate() {
        if i != 0 && i != 1 {
            part2 = integer::lcm(part2, *value);
        }
    }
    println!("Part 2 {part2}");
}
