use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;

fn main() {
    if let Ok(file) = File::open("input.txt") {
        let mut lines = io::BufReader::new(file).lines();

        for line in lines {
            if let Ok(line) = line {
                let mut fish: Vec<isize> = line
                    .split(",")
                    .into_iter()
                    .map(|n| n.parse::<isize>().unwrap())
                    .collect();
                
                for i in 0..256 {
                    let mut new_fish: Vec<isize> = Vec::new();
                    for fish in &fish {
                        match fish {
                            0 => {
                                new_fish.push(6);
                                new_fish.push(8);
                            },
                            n => {
                                new_fish.push(n-1);
                            }
                        }
                    }

                    fish = new_fish;

                    dbg!(i, fish.len());
                }

                dbg!(fish.len());
            }
        }
    }
}
