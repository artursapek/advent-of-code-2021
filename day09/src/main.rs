use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;

struct Heightmap {
    raw: Vec<Vec<isize>>,
    width: isize,
    height: isize,
}

impl Heightmap {
    fn new(raw: Vec<Vec<isize>>) -> Self {
        Self {
            width: raw[0].len() as isize,
            height: raw.len() as isize,
            raw,
        }
    }

    fn get(&self, x: isize, y: isize) -> Option<isize> {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            Some(self.raw[y as usize][x as usize])
        } else {
            None
        }
    }

    fn adjacent(&self, x: isize, y: isize) -> Vec<isize> {
        vec![
            self.get(x - 1, y - 1),
            self.get(x + 1, y - 1),
            self.get(x + 1, y + 1),
            self.get(x - 1, y + 1),
            self.get(x, y - 1),
            self.get(x, y + 1),
            self.get(x - 1, y),
            self.get(x + 1, y),
        ]
        .into_iter()
        .filter_map(|v| v)
        .collect()
    }

    fn is_low_point(&self, x: isize, y: isize) -> bool {
        let v = self.get(x, y).unwrap();
        self.adjacent(x, y)
            .into_iter()
            .filter(|a| a < &v)
            .next()
            .is_none()
    }
}

fn main() {
    if let Ok(file) = File::open("input.txt") {
        let raw: Vec<Vec<isize>> = io::BufReader::new(file)
            .lines()
            .map(|line| {
                line.unwrap()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as isize)
                    .collect()
            })
            .collect();

        let map = Heightmap::new(raw);

        let mut low_values: Vec<isize> = Vec::new();

        for x in 0..map.width {
            for y in 0..map.height {
                if map.is_low_point(x, y) {
                    low_values.push(map.get(x, y).unwrap() + 1);
                }
            }
        }

        let sum: isize = low_values.iter().sum();

        dbg!(sum);
    }
}
