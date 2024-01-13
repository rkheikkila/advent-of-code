use regex::Regex;
use std::collections::BTreeMap;
use std::io::BufRead;
use std::{fs::File, io};

fn part1() {
    let mut stacks = BTreeMap::from([
        (1, vec!["F", "D", "B", "Z", "T", "J", "R", "N"]),
        (2, vec!["R", "S", "N", "J", "H"]),
        (3, vec!["C", "R", "N", "J", "G", "Z", "F", "Q"]),
        (4, vec!["F", "V", "N", "G", "R", "T", "Q"]),
        (5, vec!["L", "T", "Q", "F"]),
        (6, vec!["Q", "C", "W", "Z", "B", "R", "G", "N"]),
        (7, vec!["F", "C", "L", "S", "N", "H", "M"]),
        (8, vec!["D", "N", "Q", "M", "T", "J"]),
        (9, vec!["P", "G", "S"]),
    ]);

    let file = File::open("input.txt").unwrap();
    let lines_iter = io::BufReader::new(file).lines();
    let lines = Vec::from_iter(lines_iter.map(|x| x.unwrap()));

    for line in lines {
        if line.contains("move") {
            let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
            let cap = re.captures(&line).unwrap();
            let amount: i32 = cap[1].parse().unwrap();
            let from: i32 = cap[2].parse().unwrap();
            let to: i32 = cap[3].parse().unwrap();

            for i in (0..amount) {
                let c = stacks.get_mut(&from).unwrap().pop().unwrap();
                stacks.get_mut(&to).unwrap().push(&c);
            }
        }
    }

    for (key, val) in stacks.iter() {
        print!("{:}", val.last().unwrap())
    }
}

fn part2() {
    let mut stacks = BTreeMap::from([
        (1, vec!["F", "D", "B", "Z", "T", "J", "R", "N"]),
        (2, vec!["R", "S", "N", "J", "H"]),
        (3, vec!["C", "R", "N", "J", "G", "Z", "F", "Q"]),
        (4, vec!["F", "V", "N", "G", "R", "T", "Q"]),
        (5, vec!["L", "T", "Q", "F"]),
        (6, vec!["Q", "C", "W", "Z", "B", "R", "G", "N"]),
        (7, vec!["F", "C", "L", "S", "N", "H", "M"]),
        (8, vec!["D", "N", "Q", "M", "T", "J"]),
        (9, vec!["P", "G", "S"]),
    ]);

    let file = File::open("input.txt").unwrap();
    let lines_iter = io::BufReader::new(file).lines();
    let lines = Vec::from_iter(lines_iter.map(|x| x.unwrap()));

    for line in lines {
        if line.contains("move") {
            let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
            let cap = re.captures(&line).unwrap();
            let amount: i32 = cap[1].parse().unwrap();
            let from: i32 = cap[2].parse().unwrap();
            let to: i32 = cap[3].parse().unwrap();

            let from_stack = stacks.get_mut(&from).unwrap();
            let mut temp: Vec<&str> = Vec::new();
            for i in (0..amount) {
                temp.push(from_stack.pop().unwrap());
            }
            let to_stack = stacks.get_mut(&to).unwrap();
            let mut temp_iter = temp.iter().rev();
            while let Some(c) = temp_iter.next() {
                to_stack.push(c);
            }
        }
    }

    for (key, val) in stacks.iter() {
        print!("{:}", val.last().unwrap());
    }
}

fn main() {
    part1();
    println!("");
    part2();
}
