use std::fs;

fn main() {
    let content = fs::read_to_string("input").unwrap();
    let mut part1 = 0;
    let mut part2 = 0;
    for line in content.lines() {
        let hist : Vec<isize> = line.split(" ").map(|c| c.parse::<isize>().unwrap()).collect();
        part1 += hist[hist.len() - 1] + lp(hist);
    }
    println!("Part 1 {part1}");
    for line in content.lines() {
        let hist: Vec<isize> = line.split(" ").map(|c| c.parse::<isize>().unwrap()).collect();
        part2 += hist[0] - backfill(&hist);
    }
    println!("Part 2 {part2}");
}

fn lp(inp: Vec<isize>) -> isize {
    let mut calc: Vec<isize> = Vec::new();
    for i in 0..(inp.len() - 1) {
        calc.push(inp[i+1] - inp[i]);
    }
    if calc.iter().filter(|c| **c == 0).count() == calc.len() {
        return 0;
    } else {
        let res = calc[calc.len()-1] + lp(calc);
        return res;
    }
}

fn backfill(inp: &Vec<isize>) -> isize {
    let mut calc: Vec<isize> = Vec::new();
    for i in 0..(inp.len() - 1) {
        calc.push(inp[i+1] - inp[i]);
    }
    if calc.iter().filter(|c| **c == 0).count() == calc.len() {
        return 0;
    } else {
        let res = calc[0] - backfill(&calc);
        return res;
    }
}
