use std::{collections::HashMap, fs};

struct Hand {
    cards: Vec<u8>,
    bid: usize,
    rank: u8,
}

fn main() {
    let content = fs::read_to_string("input").unwrap();
    let value_map: HashMap<&str, u8> =
        HashMap::from([("T", 10), ("J", 1), ("Q", 12), ("K", 13), ("A", 14)]);
    let mut hands: Vec<Hand> = Vec::new();
    for line in content.lines() {
        let cards: Vec<u8> = line
            .split(" ")
            .nth(0)
            .unwrap()
            .chars()
            .map(|c| {
                if value_map.contains_key(c.to_string().as_str()) {
                    return *value_map.get(c.to_string().as_str()).unwrap();
                }
                return u8::try_from(c.to_digit(10).unwrap()).unwrap();
            })
            .collect();
        let mut rank = 0;
        let mut first = 0;
        let jokers = u8::try_from(cards.iter().filter(|c| **c == 1).count()).unwrap();
        for (i, j) in cards.iter().enumerate() {
            if *j == first || *j == 1 {
                continue;
            }
            let count = cards[(i + 1)..].iter().filter(|c| *c == j).count();
            match count {
                4 => {
                    rank = 6;
                }
                3 => {
                    rank = 5 + jokers;
                }
                2 => {
                    if jokers > 0 {
                        rank = 4 + jokers;
                        break;
                    }
                    first = *j;
                    if rank == 1 {
                        rank = 4;
                    } else {
                        rank = 3;
                    }
                }
                1 => {
                    match rank {
                        3 => rank = 4,
                        1 => rank = 2,
                        0 => {
                            match jokers {
                                0 => rank = 1,
                                1 => rank = 3,
                                2 => rank = 5,
                                3 => rank = 6,
                                _ => panic!(),
                            }
                        }
                        _ => (),
                    }
                    first = *j;
                }
                0 => (),
                _ => panic!(),
            }
            if rank > 3 {
                break;
            }
        }
        if jokers == 5 {
            rank = 6;
        } else if jokers > 0 && rank == 0 {
            match jokers {
                1 => rank = 1,
                2 => rank = 3,
                3 => rank = 5,
                4 => rank = 6,
                _ => panic!()
            }
        }
        hands.push(Hand {
            cards: cards,
            bid: line.split(" ").nth(1).unwrap().parse::<usize>().unwrap(),
            rank: rank,
        });
    }
    hands.sort_by(|a, b| {
        if a.rank == b.rank {
            for i in 0..5 {
                if a.cards[i] != b.cards[i] {
                    return a.cards[i].cmp(&b.cards[i]);
                }
            }
        }
        return a.rank.cmp(&b.rank);
    });
    let mut part1 = 0;
    for (i, hand) in hands.iter().enumerate() {
        // println!("{} {}", hand.bid, hand.rank);
        part1 += (i + 1) * hand.bid;
    }

    println!("Part 1 {part1}");
}
