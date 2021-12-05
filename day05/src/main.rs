use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;

fn main() {
    if let Ok(file) = File::open("input.txt") {
        let mut lines = io::BufReader::new(file).lines();

        #[derive(PartialEq, Eq, Hash, Debug)]
        struct Point {
            x: isize,
            y: isize,
        }

        let mut seafloor: HashMap<Point, isize> = HashMap::new();

        fn parse_point(p: &str) -> Point {
            let mut values = p.split(",").into_iter();
            let x = values.next().unwrap().parse::<isize>().unwrap();
            let y = values.next().unwrap().parse::<isize>().unwrap();
            Point { x, y }
        };

        for line in lines {
            if let Ok(line) = line {
                let mut points = line.split(" -> ").into_iter();
                let a = parse_point(points.next().unwrap());
                let b = parse_point(points.next().unwrap());

                let points: Vec<Point> = {
                    if a.x == b.x {
                        Range {
                            start: a.y.min(b.y),
                            end: a.y.max(b.y) + 1,
                        }
                        .into_iter()
                        .map(|y| Point { x: a.x, y })
                        .collect()
                    } else if a.y == b.y {
                        Range {
                            start: a.x.min(b.x),
                            end: a.x.max(b.x) + 1,
                        }
                        .into_iter()
                        .map(|x| Point { y: a.y, x })
                        .collect()
                    } else {
                        let (min, max) = {
                            if a.x < b.x {
                                (a, b)
                            } else {
                                (b, a)
                            }
                        };
                        let slope = (max.y - min.y) / (max.x - min.x);

                        (min.x..max.x + 1)
                            .into_iter()
                            .map(|x| {
                                let y = min.y + ((x - min.x) * slope);
                                Point { x, y }
                            })
                            .collect()
                    }
                };

                for point in points {
                    let n = seafloor.get(&point).unwrap_or(&0);
                    seafloor.insert(point, n + 1);
                }
            }
        }

        let mut total = 0;
        let mut total_overlapping = 0;

        for (point, n) in seafloor.iter() {
            total += 1;
            if n > &1 {
                total_overlapping += 1;
            }
        }

        dbg!(total, total_overlapping);
    }
}
