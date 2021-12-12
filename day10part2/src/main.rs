use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;

fn main() {
    if let Ok(file) = File::open("input.txt") {
        let mut scores: Vec<i64> = Vec::new();

        for line in io::BufReader::new(file).lines() {
            if let Ok(line) = line {
                let mut opening_chars = Vec::new();

                let mut valid = true;

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
                                _ => {
                                    valid = false;
                                    break;
                                }
                            }
                        }
                        _ => {}
                    }
                }

                if valid {

                    dbg!(&line, &opening_chars);

                    let mut score: i64 = 0;
                    for c in opening_chars.iter().rev() {
                        score *= 5;
                        score += match c {
                            '(' => 1,
                            '[' => 2,
                            '{' => 3,
                            '<' => 4,
                            _ => 0,
                        };
                    }
                    dbg!(opening_chars, score);

                    scores.push(score);
                }
            }
        }

        scores.sort();

        dbg!(scores[scores.len()/2]);
    }
}
