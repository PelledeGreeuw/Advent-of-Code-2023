use regex::Regex;
use std::{cmp, collections::HashMap, fs};

struct Entry {
    dest_start: usize,
    src_start: usize,
    length: usize,
}

struct SourceDest {
    dest: String,
    src: String,
    entries: Vec<Entry>,
}

fn main() {
    let content = fs::read_to_string("input").unwrap();
    let seeds: Vec<usize> = content
        .lines()
        .nth(0)
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|c| c.parse::<usize>().unwrap())
        .collect();
    let mut source_dest: HashMap<String, SourceDest> = HashMap::new();
    let matcher =
        Regex::new(r"(?<src>\w+)-to-(?<dest>\w+) map:\n(?<map>(?:(?:\d+ ?)+\n)+)").unwrap();
    for (_, [src, dest, map]) in matcher.captures_iter(&content).map(|c| c.extract()) {
        let mut entries: Vec<Entry> = Vec::new();
        for line in map.lines() {
            let nm: Vec<usize> = line
                .split(" ")
                .map(|c| c.parse::<usize>().unwrap())
                .collect();
            entries.push(Entry {
                dest_start: nm[0],
                src_start: nm[1],
                length: nm[2],
            })
        }
        entries.sort_by(|a, b| a.dest_start.cmp(&b.dest_start));
        source_dest.insert(
            src.to_string(),
            SourceDest {
                dest: dest.to_string(),
                src: src.to_string(),
                entries: entries,
            },
        );
    }
    let mut part1 = usize::MAX;
    for seed in &seeds {
        let mut src = &"seed".to_string();
        let mut step = *seed;
        while src != "location" {
            let m = source_dest.get(src).unwrap();

            for entry in &m.entries {
                if entry.src_start <= step && step <= entry.src_start + entry.length {
                    step = entry.dest_start + (step - &entry.src_start);
                    break;
                }
            }

            src = &m.dest;
        }
        if part1 > step {
            part1 = step;
        }
    }

    println!("Part 1 {part1}");

    let mut entries: Vec<Entry> = Vec::new();
    for i in 0..(seeds.len() / 2) {
        entries.push(Entry {
            dest_start: seeds[i * 2],
            src_start: seeds[i * 2],
            length: seeds[i * 2 + 1],
        });
    }
    entries.sort_by(|a, b| b.dest_start.cmp(&a.dest_start));
    source_dest.insert(
        "start".to_string(),
        SourceDest {
            dest: "seed".to_string(),
            src: "start".to_string(),
            entries: entries,
        },
    );

    let mut seed =
        usize::try_from(find(0, usize::MAX, "location".to_string(), &source_dest)).unwrap();
    let mut src = &"seed".to_string();

    while src != "location" {
        let m = source_dest.get(src).unwrap();

        for entry in &m.entries {
            if entry.src_start <= seed && seed <= entry.src_start + entry.length {
                seed = entry.dest_start + (seed - &entry.src_start);
                break;
            }
        }

        src = &m.dest;
    }
    println!("Part 2 {seed}");
}

fn find(start: usize, length: usize, src: String, maps: &HashMap<String, SourceDest>) -> isize {
    let mut part_start = start;
    let mut part_length = length;

    let mut source_key = "".to_string();
    for sd in maps.values() {
        if sd.dest == src {
            source_key = sd.src.to_string();
            break;
        }
    }
    for entry in &maps.get(&source_key).unwrap().entries {
        let mut dest_start;
        let mut dest_length;
        if part_length == 0 {
            break;
        }

        if part_start >= entry.dest_start + entry.length {
            continue;
        }
        if part_start < entry.dest_start {
            if &maps.get(&source_key).unwrap().src == "start" {
                continue;
            }
            dest_start = part_start;
            dest_length = cmp::min(part_length, entry.dest_start - part_start);
            let result = find(
                dest_start,
                dest_length,
                maps.get(&source_key).unwrap().src.to_string(),
                maps,
            );
            if result != -1 {
                return result;
            }
            part_length -= dest_length;
            part_start += dest_length;
        }
        if part_length == 0 {
            continue;
        }

        if &maps.get(&source_key).unwrap().src == "start" {
            return isize::try_from(part_start).unwrap();
        }
        dest_start = entry.src_start + (part_start - entry.dest_start);
        dest_length = cmp::min(part_length, entry.length - (part_start - entry.dest_start));

        if &maps.get(&source_key).unwrap().src == "start" {
            continue;
        }
        let result = find(
            dest_start,
            dest_length,
            maps.get(&source_key).unwrap().src.to_string(),
            maps,
        );
        if result != -1 {
            return result;
        }
        part_length -= dest_length;
        part_start += dest_length;
    }

    if part_length != 0 {
        if &maps.get(&source_key).unwrap().src == "start" {
            return -1;
        }
        return find(
            part_start,
            part_length,
            maps.get(&source_key).unwrap().src.to_string(),
            maps,
        );
    }
    return -1;
}
