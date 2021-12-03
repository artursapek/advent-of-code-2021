use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    if let Ok(file) = File::open("input.txt") {
        let lines = io::BufReader::new(file).lines();

        let mut bits: Vec<i32> = Vec::new();

        for line in lines {
            if let Ok(line) = line {
                if bits.len() == 0 {
                    bits = vec![0; line.len()];
                }

                for (index, c) in line.chars().enumerate() {
                    match c {
                        '1' => bits[index] += 1,
                        '0' => bits[index] -= 1,
                        _ => {}
                    }
                }
            }
        }

        let epsilon = String::from_iter(bits.iter().map(|bit| if *bit < 0 { '0' } else { '1' }));
        let gamma = String::from_iter(bits.iter().map(|bit| if *bit < 0 { '1' } else { '0' }));

        let epsilon_int = i32::from_str_radix(&epsilon, 2).unwrap();
        let gamma_int = i32::from_str_radix(&gamma, 2).unwrap();

        dbg!(epsilon_int * gamma_int);
    }
}
