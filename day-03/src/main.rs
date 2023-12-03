use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::str;

fn main() {
    let content = fs::read_to_string("input").unwrap();
    let lines: Vec<&str> = content.lines().collect();
    let matcher = Regex::new(r"\d+").unwrap();
    let mut gears: HashMap<usize, HashMap<usize, Vec<u32>>> = HashMap::new();
    let mut total = 0;
    for (i, line) in content.lines().enumerate() {
        for m in matcher.find_iter(line) {
            let mut found = false;
            let mut x_start = m.start();
            if x_start > 0 {
                x_start -= 1;
            }
            let mut x_end = m.end();
            if x_end < line.len() - 1 {
                x_end += 1;
            }
            let mut y_start = i;
            if y_start > 0 {
                y_start -= 1;
            }
            let mut y_end = i + 1;
            if y_end < lines.len() - 1 {
                y_end += 1;
            }
            for y in y_start..y_end {
                for x in x_start..x_end {
                    if !".0123456789".contains(&lines[y][x..x + 1]) {
                        found = true;
                    }

                    if &lines[y][x..x + 1] == "*" {
                        if !gears.contains_key(&y) {
                            gears.insert(y, HashMap::new());
                        }
                        if !gears.get(&y).unwrap().contains_key(&x) {
                            gears.get_mut(&y).unwrap().insert(x, Vec::new());
                        }
                        gears
                            .get_mut(&y)
                            .unwrap()
                            .get_mut(&x)
                            .unwrap()
                            .push(m.as_str().parse::<u32>().unwrap());
                    }
                }
            }
            if found {
                total += m.as_str().parse::<u32>().unwrap();
            }
        }
    }
    println!("part 1: {total}");
    let mut part2 = 0;
    for x in gears.values() {
        for gear in x.values() {
            if gear.len() == 2 {
                part2 += gear[0] * gear[1];
            }
        }
    }
    println!("part 2: {part2}");
}
