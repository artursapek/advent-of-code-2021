use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

/*

      0:      1:      2:      3:      4:
     aaaa    ....    aaaa    aaaa    ....
    b    c  .    c  .    c  .    c  b    c
    b    c  .    c  .    c  .    c  b    c
     ....    ....    dddd    dddd    dddd
    e    f  .    f  e    .  .    f  .    f
    e    f  .    f  e    .  .    f  .    f
     gggg    ....    gggg    gggg    ....

      5:      6:      7:      8:      9:
     aaaa    aaaa    aaaa    aaaa    aaaa
    b    .  b    .  .    c  b    c  b    c
    b    .  b    .  .    c  b    c  b    c
     dddd    dddd    ....    dddd    dddd
    .    f  e    f  .    f  e    f  .    f
    .    f  e    f  .    f  e    f  .    f
     gggg    gggg    ....    gggg    gggg

*/

struct Descrambler {
    mappings: HashMap<String, isize>,
}

#[derive(Debug)]
struct Digit(HashSet<char>);

impl Descrambler {
    fn new() -> Self {
        Self {
            mappings: HashMap::new(),
        }
    }

    fn read(&mut self, digits: Vec<Digit>) {
        let one = digits.iter().find(|d| d.0.len() == 2).unwrap();
        let seven = digits.iter().find(|d| d.0.len() == 3).unwrap();
        let four = digits.iter().find(|d| d.0.len() == 4).unwrap();
        let eight = digits.iter().find(|d| d.0.len() == 7).unwrap();

        let six = digits
            .iter()
            .find(|d| d.0.len() == 6 && one.0.intersection(&d.0).count() == 1)
            .unwrap();

        let nine = digits
            .iter()
            .find(|d| d.0.len() == 6 && four.0.intersection(&d.0).count() == 4)
            .unwrap();

        let zero = digits
            .iter()
            .find(|d| {
                d.0.len() == 6
                    && nine.0.intersection(&d.0).count() == 5
                    && six.0.intersection(&d.0).count() == 5
            })
            .unwrap();

        let three = digits
            .iter()
            .find(|d| d.0.len() == 5 && one.0.intersection(&d.0).count() == 2)
            .unwrap();

        let five = digits
            .iter()
            .find(|d| {
                d.0.len() == 5
                    && four.0.intersection(&d.0).count() == 3
                    && one.0.intersection(&d.0).count() == 1
            })
            .unwrap();

        let two = digits
            .iter()
            .find(|d| {
                d.0.len() == 5
                    && four.0.intersection(&d.0).count() == 2
                    && three.0.intersection(&d.0).count() != 5
            })
            .unwrap();

        if two.0.intersection(&five.0).count() == 5 {
            dbg!("two and five are the same");
        }
        if two.0.intersection(&three.0).count() == 5 {
            dbg!("two and three are the same");
        }
        if five.0.intersection(&three.0).count() == 5 {
            dbg!("five and three are the same");
        }

        //dbg!(zero, one, two, three, four, five, six, seven, eight, nine);

        self.mappings.insert(zero.to_string(), 0);
        self.mappings.insert(one.to_string(), 1);
        self.mappings.insert(two.to_string(), 2);
        self.mappings.insert(three.to_string(), 3);
        self.mappings.insert(four.to_string(), 4);
        self.mappings.insert(five.to_string(), 5);
        self.mappings.insert(six.to_string(), 6);
        self.mappings.insert(seven.to_string(), 7);
        self.mappings.insert(eight.to_string(), 8);
        self.mappings.insert(nine.to_string(), 9);
    }

    fn digit(&self, scrambled: &Digit) -> isize {
        match self.mappings.get(&scrambled.to_string()) {
            Some(val) => *val,
            None => {
                dbg!(scrambled);
                0
            }
        }
    }
}

impl From<&str> for Digit {
    fn from(s: &str) -> Digit {
        let mut set = HashSet::new();
        for c in s.chars() {
            set.insert(c);
        }
        Self(set)
    }
}

impl ToString for Digit {
    fn to_string(&self) -> String {
        let mut chars: Vec<char> = self.0.iter().copied().collect();
        chars.sort_by(|a, b| b.cmp(a));
        String::from_iter(chars)
    }
}

fn main() {
    if let Ok(file) = File::open("input.txt") {
        let lines = io::BufReader::new(file).lines();

        let mut sum = 0;

        for line in lines {
            if let Ok(line) = line {
                let mut descrambler = Descrambler::new();

                let mut split = line.split(" | ");
                let random: Vec<Digit> = split
                    .next()
                    .unwrap()
                    .split(" ")
                    .map(|digit| Digit::from(digit))
                    .collect();

                let output: Vec<Digit> = split
                    .next()
                    .unwrap()
                    .split(" ")
                    .map(|digit| Digit::from(digit))
                    .collect();

                descrambler.read(random);

                let output: Vec<isize> = output
                    .iter()
                    .map(|digit| descrambler.digit(digit))
                    .collect();
                let output = (output[0] * 1000) + (output[1] * 100) + (output[2] * 10) + output[3];
                
                sum += output;
            }
        }

        dbg!(sum);
    }
}
