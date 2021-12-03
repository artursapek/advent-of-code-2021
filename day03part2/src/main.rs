use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    if let Ok(file) = File::open("input.txt") {
        let lines: Vec<String> = io::BufReader::new(file)
            .lines()
            .filter_map(|line| {
                if let Ok(line) = line {
                    Some(line)
                } else {
                    None
                }
            })
            .collect();

        let mut bits: Vec<i32> = Vec::new();

        let mut lines_oxy = lines.clone();
        let mut lines_co2 = lines.clone();

        let mut bit_position: usize = 0;
        while lines_oxy.len() > 1 {
            let mut bit_accum: isize = 0;

            for line in &lines_oxy {
                match line.chars().nth(bit_position) {
                    Some('1') => bit_accum += 1,
                    Some('0') => bit_accum -= 1,
                    _ => {}
                }
            }
            lines_oxy = lines_oxy
                .into_iter()
                .filter(|line| {
                    if bit_accum >= 0 {
                        line.chars().nth(bit_position) == Some('1')
                    } else {
                        line.chars().nth(bit_position) == Some('0')
                    }
                })
                .collect();

            bit_position += 1;
        }

        let mut bit_position: usize = 0;
        while lines_co2.len() > 1 {
            let mut bit_accum: isize = 0;

            for line in &lines_co2 {
                match line.chars().nth(bit_position) {
                    Some('1') => bit_accum += 1,
                    Some('0') => bit_accum -= 1,
                    _ => {}
                }
            }
            lines_co2 = lines_co2
                .into_iter()
                .filter(|line| {
                    if bit_accum >= 0 {
                        line.chars().nth(bit_position) == Some('0')
                    } else {
                        line.chars().nth(bit_position) == Some('1')
                    }
                })
                .collect();

            bit_position += 1;
        }

        dbg!(&lines_oxy);
        dbg!(&lines_co2);

        let oxy = i32::from_str_radix(&lines_oxy.get(0).unwrap(), 2).unwrap();
        let co2 = i32::from_str_radix(&lines_co2.get(0).unwrap(), 2).unwrap();

        dbg!(oxy * co2);
    }
}
