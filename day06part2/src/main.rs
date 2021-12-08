use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;

fn main() {
    if let Ok(file) = File::open("input.txt") {
        let mut lines = io::BufReader::new(file).lines();

        for line in lines {
            if let Ok(line) = line {
                let mut all_fish: Vec<usize> = line
                    .split(",")
                    .into_iter()
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect();

                let mut fish_map: HashMap<usize, usize> = HashMap::new();

                for fish in &all_fish {
                    match fish_map.get(&fish) {
                        Some(n) => {
                            fish_map.insert(*fish, n + 1);
                        }
                        None => {
                            fish_map.insert(*fish, 1);
                        }
                    }
                }

                for i in 0..256 {
                    let mut new_map: HashMap<usize, usize> = HashMap::new();

                    let fish_to_give_birth = fish_map.get(&0).unwrap_or(&0);

                    for i in 1..9 {
                        let num = fish_map.get(&i).unwrap_or(&0);

                        let next_age = i - 1;

                        if next_age == 6 {
                            let num = num + fish_to_give_birth;
                            new_map.insert(next_age, num);
                        } else {
                            new_map.insert(next_age, *num);
                        }
                    }
                    new_map.insert(8, *fish_to_give_birth);

                    fish_map = new_map;
                }


                let sum: usize = fish_map.values().sum();

                dbg!(sum);
            }
        }
    }
}
