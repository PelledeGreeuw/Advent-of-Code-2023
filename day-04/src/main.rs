use regex::Regex;
use std::fs;

fn main() {
    let content = fs::read_to_string("input").unwrap();
    let matcher = Regex::new(r"Card +\d+: ((?: ?\d+ )+)\| ((?: ?\d+ ?)+)").unwrap();
    let mut total = 0;
    let mut prop : Vec<Vec<usize>> = Vec::new();

    for line in content.lines() {
        let groups = matcher.captures(line).unwrap();
        let mine: Vec<u32> = groups[1]
            .split(" ")
            .filter(|c| c.len() > 0)
            .map(|c| c.parse::<u32>().unwrap())
            .collect();
        let winning: Vec<u32> = groups[2]
            .split(" ")
            .filter(|c| c.len() > 0)
            .map(|c| c.parse::<u32>().unwrap())
            .collect();
        let mut points = 0;
        let mut winnings = 0;
        for n in mine {
            if winning.contains(&n) {
                winnings += 1;
                if points == 0{
                    points = 1;
                } else {
                    points = points * 2;
                }
            }
        }
        let mut d : Vec<usize> = Vec::new();
        d.push(1);
        d.push(winnings);
        prop.push(d);
        total += points;
    }
    println!("part 1: {total}");
    let mut part2 = 0;

    for i in 0..prop.len() {
        part2 += prop[i][0];
        if prop[i][1] > 0 {
            for j in 1..(prop[i][1]+1) {
                prop[i+j][0] += prop[i][0];
            }
        }
    }
    println!("Part 2 {part2}");
}
