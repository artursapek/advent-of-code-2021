use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;

fn main() {
    if let Ok(file) = File::open("input.txt") {
        let mut lines = io::BufReader::new(file).lines();

        let mut sum = 0;

        for line in lines {
            if let Ok(line) = line {
                let mut split = line.split(" | ");
                let random = split.next().unwrap();
                let output = split.next().unwrap();

                let outputs = output.split(" ");
                for val in outputs {
                    match val.len() {
                        2 | 3 | 4 | 7 => sum += 1,
                        _ => {}
                    }
                }
            }
        }

        dbg!(sum);
    }
}
