use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    if let Ok(file) = File::open("input.txt") {
        let lines = io::BufReader::new(file).lines();

        let mut window = Vec::new();
        let mut prev_sum: Option<i32> = None;
        let mut total = 0;

        for line in lines {
            if let Ok(line) = line {
                let value: i32 = line.parse().unwrap();
                window.push(value);

                if window.len() > 3 {
                    window.remove(0);
                }

                if window.len() == 3 {
                    let sum = window.iter().sum::<i32>();

                    match (prev_sum, sum) {
                        (Some(prev_sum), sum) if sum > prev_sum => total += 1,
                        _ => {}
                    }

                    prev_sum = Some(sum);
                }
            }
        }

        println!("{}", total);
    }
}
