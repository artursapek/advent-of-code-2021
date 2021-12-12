use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;

struct Octos {
    raw: Vec<Vec<isize>>,
    width: isize,
    height: isize,

    flashes: isize,

    already_flashed: HashSet<(isize, isize)>,
}

#[derive(Debug)]
struct Octo {
    x: isize,
    y: isize,
    value: isize,
}

impl Octos {
    fn new(raw: Vec<Vec<isize>>) -> Self {
        Self {
            width: raw[0].len() as isize,
            height: raw.len() as isize,
            raw,
            flashes: 0,
            already_flashed: HashSet::new(),
        }
    }

    fn get(&self, x: isize, y: isize) -> Option<Octo> {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            Some(Octo {
                x,
                y,
                value: self.raw[y as usize][x as usize],
            })
        } else {
            None
        }
    }

    fn adjacent(&self, x: isize, y: isize) -> Vec<Octo> {
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

    fn step(&mut self) {
        self.already_flashed = HashSet::new();
        for y in 0..self.height {
            for x in 0..self.width {
                self.increment(x, y);
            }
        }
    }

    fn increment(&mut self, x: isize, y: isize) {
        if !self.already_flashed.contains(&(x, y)) {
            if let Some(octo) = self.get(x, y) {
                match octo.value {
                    9 => {
                        // Flash
                        self.raw[y as usize][x as usize] = 0;
                        self.already_flashed.insert((x, y));
                        for neighbor in self.adjacent(x, y) {
                            self.increment(neighbor.x, neighbor.y);
                        }

                        self.flashes += 1;

                    }
                    n => {
                        self.raw[y as usize][x as usize] = n + 1;
                    }
                }
            }
        }
    }

    fn print(&self) {
        for line in &self.raw {
            for c in line {
                print!("{}", c);
            }
            print!("{}", '\n');
        }
        print!("{}", '\n');
        print!("{}", '\n');
    }

    fn all_flashed(&self) -> bool {
        self.already_flashed.len() == (self.width * self.height) as usize
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

        let mut octos = Octos::new(raw);

        for i in 0..10000 {
            octos.step();
            
            if octos.all_flashed() {
                dbg!(i);
                octos.print();
                break;
            }
        }

        dbg!(octos.flashes);
    }
}
