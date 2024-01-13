use std::cmp;
use std::io::BufRead;
use std::{fs::File, io};

struct Interval {
    low: i32,
    high: i32,
}

impl Interval {
    fn from_string(s: &str) -> Interval {
        let mut split = s.split("-");
        let low: i32 = split.next().unwrap().parse().unwrap();
        let high: i32 = split.next().unwrap().parse().unwrap();
        Interval {
            low: low,
            high: high,
        }
    }

    fn contains(&self, other: &Interval) -> bool {
        self.low <= other.low && self.high >= other.high
    }

    // a ----- b
    //    d ----- e
    fn intersection(&self, other: &Interval) -> Option<Interval> {
        if other.low > self.high || self.low > other.high {
            return None;
        } else {
            let low = cmp::max(self.low, other.low);
            let high = cmp::min(self.high, other.high);
            return Some(Interval {
                low: low,
                high: high,
            });
        }
    }
}

fn check_intervals(line: &String) -> bool {
    let mut split = line.split(",");
    let first_interval = Interval::from_string(split.next().unwrap());
    let second_interval = Interval::from_string(split.next().unwrap());

    first_interval.contains(&second_interval) || second_interval.contains(&first_interval)
}

fn check_intersection(line: &String) -> bool {
    let mut split = line.split(",");
    let first_interval = Interval::from_string(split.next().unwrap());
    let second_interval = Interval::from_string(split.next().unwrap());

    match first_interval.intersection(&second_interval) {
        Some(_) => true,
        None => false,
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let lines_iter = io::BufReader::new(file).lines();
    let lines = Vec::from_iter(lines_iter.map(|x| x.unwrap()));

    let count = lines
        .iter()
        .map(|x| check_intervals(&x))
        .filter(|x| *x)
        .count();

    println!("{:}", count);

    let intersect_count = lines
        .iter()
        .map(|x| check_intersection(&x))
        .filter(|x| *x)
        .count();

    println!("{:}", intersect_count);
}
