use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    if let Ok(file) = File::open("input.txt") {
        let lines = io::BufReader::new(file).lines();

        let mut depth: i32 = 0;
        let mut horizontal: i32 = 0;
        let mut aim: i32 = 0;

        for line in lines {
            if let Ok(line) = line {
                let mut parts = line.split(" ");

                let command = parts.next().unwrap();
                let value = parts.next().unwrap().parse::<i32>().unwrap();

                match command {
                    "up" => {
                        aim -= value;
                    }
                    "down" => {
                        aim += value;
                    }
                    "forward" => {
                        horizontal += value;
                        depth += (aim * value);
                    }
                    _ => {}
                }
            }
        }

        dbg!(depth);
        dbg!(horizontal);
        dbg!(depth * horizontal);
    }
}
