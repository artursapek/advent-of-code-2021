use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;

fn main() {
    if let Ok(file) = File::open("input.txt") {
        let mut lines = io::BufReader::new(file).lines();

        let mut crabs: Vec<isize> = lines
            .next()
            .unwrap()
            .unwrap()
            .split(",")
            .into_iter()
            .map(|n| n.parse::<isize>().unwrap())
            .sorted()
            .collect();

        let sum: isize = crabs.iter().sum();

        let min: isize = *crabs.iter().min().unwrap();
        let max: isize = *crabs.iter().max().unwrap();
        let mean = ((sum as f32) / (crabs.len() as f32)).floor();
        let median = crabs[crabs.len() / 2];

        let min_usage = (min..max + 1).into_iter().map(|posn| {
            let fuel_usage: isize = crabs
                .iter()
                .map(|crab| {
                    let steps = (crab - (posn as isize)).abs();
                    (1..(steps + 1)).into_iter().fold(0, |acc, x| acc + x)
                })
                .sum();

            fuel_usage
        }).min();

        dbg!(min_usage);


    }
}
