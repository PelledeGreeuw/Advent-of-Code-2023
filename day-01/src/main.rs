use std::fs;

fn main() {
    let content = fs::read_to_string("input")
    .expect("Couldn't read file");
    let lines = content.split("\n");
    let mut total = 0;

    for line in lines {
        let mut dig = "".to_owned();
        let mut first = 300;
        let mut replaced = line.to_owned();
        let matches = [("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"), ("five", "5"), ("six", "6"), ("seven", "7"),("eight", "8"),("nine", "9")];
        for (m, rep) in matches {
            match line.find(m) {
                Some(p) => {
                    if p < first {
                        first = p;
                        replaced = line.replace(m, rep);
                    }
                },
                None => continue
            }
        }

        for c in replaced.chars() {
            match c.to_string().parse::<i32>() {
                Ok(num) => {
                    dig.push_str(&num.to_string());
                    break;
                },
                Err(_) => continue
            }
        }

        first = 300;
        let rev_line : String = line.chars().rev().collect();
        replaced = rev_line.clone();
        for (m, rep) in matches {
            match rev_line.find(&m.chars().rev().collect::<String>()) {
                Some(p) => {
                    if p < first {
                        first = p;
                        replaced = rev_line.replace(&m.chars().rev().collect::<String>(), rep);
                    }
                },
                None => continue
            }
        } 
        for c in replaced.chars() {
            match c.to_string().parse::<i32>() {
                Ok(num) => {
                    dig.push_str(&num.to_string());
                    break;
                },
                Err(_) => continue
            }
        }
        println!("{line} => {dig}");
        total += dig.parse::<i32>().unwrap();
    }
    println!("{total}");
}
