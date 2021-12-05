use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Board {
    rows: Vec<Vec<BoardNumber>>,
}

#[derive(Debug, Clone, Copy)]
enum BoardNumber {
    Waiting(usize),
    Drawn(usize),
}

impl Board {
    fn new(rows: Vec<Vec<usize>>) -> Self {
        Board {
            rows: rows
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|number| BoardNumber::Waiting(*number))
                        .collect()
                })
                .collect(),
        }
    }

    fn draw(&mut self, drawn: usize) {
        for row in &mut self.rows {
            for number in row {
                number.draw(drawn);
            }
        }
    }

    fn is_winning(&self) -> bool {
        for row in &self.rows {
            if row.iter().filter(|n| n.is_waiting()).next().is_none() {
                return true;
            }
        }
        for i in 0..5 {
            let mut winning = true;
            for row in &self.rows {
                if row[i].is_waiting() {
                    winning = false;
                    break;
                }
            }
            if winning {
                return true;
            }
        }
        false
    }
}

impl BoardNumber {
    fn draw(&mut self, drawn: usize) {
        *self = match self {
            BoardNumber::Waiting(n) if *n == drawn => BoardNumber::Drawn(*n),
            _ => *self,
        };
    }

    fn is_waiting(&self) -> bool {
        match self {
            BoardNumber::Waiting(_) => true,
            _ => false,
        }
    }
}

fn main() {
    if let Ok(file) = File::open("input.txt") {
        let mut lines = io::BufReader::new(file).lines();

        let numbers: Vec<usize> = lines
            .next()
            .unwrap()
            .unwrap()
            .split(",")
            .into_iter()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        // Eat newline
        lines.next();

        // Collect boards
        let mut boards: Vec<Board> = Vec::new();
        let mut rows: Vec<Vec<usize>> = Vec::new();

        for line in lines {
            if let Ok(line) = line {
                if line.len() > 1 {
                    let numbers = line
                        .split(" ")
                        .into_iter()
                        .filter_map(|n| n.parse::<usize>().ok())
                        .collect();
                    rows.push(numbers);
                    // Process row
                } else {
                    // Persist board and start a new one
                    boards.push(Board::new(rows));
                    rows = Vec::new();
                }
            }
        }

        'numberLoop: for draw in numbers {
            for board in &mut boards {
                board.draw(draw);

                if board.is_winning() {
                    let undrawn_numbers: Vec<usize> = board
                        .rows
                        .iter()
                        .map(|row| {
                            row.iter()
                                .filter_map(|n| match n {
                                    BoardNumber::Waiting(n) => Some(n),
                                    _ => None,
                                })
                                .copied()
                                .collect::<Vec<usize>>()
                        })
                        .flatten()
                        .collect::<Vec<usize>>();

                    let sum: usize = undrawn_numbers.iter().sum();

                    dbg!(sum * draw);

                }
            }

            boards = boards
                .into_iter()
                .filter(|board| !board.is_winning())
                .collect();
        }
    }
}
