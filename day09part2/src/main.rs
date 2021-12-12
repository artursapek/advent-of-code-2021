use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
struct Coord {
    x: isize,
    y: isize,
    val: isize,
}

struct Heightmap {
    raw: Vec<Vec<isize>>,
    width: isize,
    height: isize,

    basins: Vec<Vec<Coord>>,
    visited: HashSet<Coord>,
}

impl Heightmap {
    fn new(raw: Vec<Vec<isize>>) -> Self {
        Self {
            width: raw[0].len() as isize,
            height: raw.len() as isize,
            raw,

            basins: Vec::new(),
            visited: HashSet::new(),
        }
    }

    fn get(&self, x: isize, y: isize) -> Option<Coord> {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            Some(Coord {
                x,
                y,
                val: self.raw[y as usize][x as usize],
            })
        } else {
            None
        }
    }

    fn adjacent(&self, coord: &Coord) -> Vec<Coord> {
        let x = coord.x;
        let y = coord.y;
        vec![
            self.get(x, y - 1),
            self.get(x, y + 1),
            self.get(x - 1, y),
            self.get(x + 1, y),
        ]
        .into_iter()
        .filter_map(|v| v)
        .collect()
    }

    fn scan_for_basin(&mut self, x: isize, y: isize) {
        if let Some(coord) = self.get(x, y) {
            let mut basin: Vec<Coord> = Vec::new();
            let mut to_visit = vec![coord.clone()];

            while to_visit.len() > 0 {
                if let Some(coord) = to_visit.pop() {
                    if !self.visited.contains(&coord) {
                        match coord.val {
                            9 => {
                                self.visited.insert(coord.clone());
                            }

                            _ => {
                                basin.push(coord.clone());
                                //dbg!(basin.len());
                                self.visited.insert(coord.clone());
                                to_visit.append(&mut self.adjacent(&coord));
                                //dbg!(to_visit.len());
                            }
                        }
                    }
                }
            }

            if basin.len() > 0 {
                println!("Pushing basin with length of {}", basin.len());

                self.basins.push(basin);
            }
        }
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

        let mut map = Heightmap::new(raw);

        for x in 0..map.width {
            for y in 0..map.height {
                map.scan_for_basin(x, y);
            }
        }

        let mut basins: Vec<usize> = map
            .basins
            .iter()
            .map(|b| b.len())
            .collect();

        basins.sort();

        dbg!(basins.pop().unwrap() * basins.pop().unwrap() * basins.pop().unwrap());
    }
}
