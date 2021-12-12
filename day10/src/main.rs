use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;

fn main() {
    if let Ok(file) = File::open("input.txt") {

        let mut score = 0;

        for line in io::BufReader::new(file).lines() {
            if let Ok(line) = line {
                let mut opening_chars = Vec::new();

                for c in line.chars() {
                    match c {
                        '[' | '{' | '<' | '(' => {
                            opening_chars.push(c);
                        }
                        ']' | '}' | '>' | ')' => {
                            match (opening_chars.last().unwrap(), c) {
                                ('[', ']') | ('{', '}') | ('<', '>') | ('(', ')') => {
                                    // OK, ignore
                                    opening_chars.pop();
                                }
                                // Invalid closing chars
                                (_, ')') => {
                                    score += 3;
                                    break;
                                }
                                (_, ']') => {
                                    score += 57;
                                    break;
                                }
                                (_, '}') => {
                                    score += 1197;
                                    break;
                                }
                                (_, '>') => {
                                    score += 25137;
                                    break;
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            }
            dbg!(score);
        }

        dbg!(score);
    }
}
