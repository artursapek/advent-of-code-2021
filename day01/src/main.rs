use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    if let Ok(file) = File::open("input.txt") {
        let lines = io::BufReader::new(file).lines();

        let mut prev: Option<i32> = None;
        let mut total = 0;

        for line in lines {
            if let Ok(line) = line {
                let value: i32 = line.parse().unwrap();
                match (prev, value) {
                    (Some(prev), value) if value > prev => total += 1,
                    _ => {}
                }
                prev = Some(value);
            }
        }

        println!("{}", total);
    }
}
