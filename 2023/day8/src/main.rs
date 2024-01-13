use num::integer::lcm;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let answer1 = part1(input);
    println!("{}", answer1);
    let answer2 = part2(input);
    println!("{}", answer2);
}

fn part1(input: &str) -> u32 {
    let mut lines = input.split("\n");
    let mut instructions = lines.next().unwrap().chars().cycle();
    let locations: Vec<(&str, (&str, &str))> = lines
        .map(|line| line.split_once(" = ").unwrap())
        .map(|(start, next)| {
            let start_clean = start.trim();
            let (left, right) = next
                .trim_matches('(')
                .trim_matches(')')
                .split_once(", ")
                .unwrap();
            (start_clean, (left, right))
        })
        .collect();

    let map: HashMap<&str, (&str, &str)> = HashMap::from_iter(locations.clone());

    let mut location = "AAA";
    let mut steps = 0;

    while location != "ZZZ" {
        let next_instruction = instructions.next().unwrap();
        let (left, right) = map.get(&location).expect("wtf");
        if next_instruction == 'L' {
            location = left;
        } else {
            location = right;
        }
        steps += 1;
    }
    return steps;
}

fn search(start: &str, instructions: &Vec<char>, map: &HashMap<&str, (&str, &str)>) -> u32 {
    let mut steps = 0;
    let mut location = start;

    let mut instructions_iter = instructions.iter().cycle();

    while !location.ends_with('Z') {
        let next_instruction = instructions_iter.next().unwrap().clone();
        let (left, right) = map.get(&location).expect("wtf");
        if next_instruction == 'L' {
            location = left;
        } else {
            location = right;
        }
        steps += 1;
    }
    return steps;
}

fn part2(input: &str) -> u64 {
    let mut lines = input.split("\n");
    let instructions: Vec<char> = lines.next().unwrap().chars().collect();
    let locations: Vec<(&str, (&str, &str))> = lines
        .map(|line| line.split_once(" = ").unwrap())
        .map(|(start, next)| {
            let start_clean = start.trim();
            let (left, right) = next
                .trim_matches('(')
                .trim_matches(')')
                .split_once(", ")
                .unwrap();
            (start_clean, (left, right))
        })
        .collect();

    let map: HashMap<&str, (&str, &str)> = HashMap::from_iter(locations.clone());

    let locations: Vec<&str> = locations
        .iter()
        .map(|(k, _)| *k)
        .filter(|k| k.ends_with('A'))
        .collect();

    let lengths: Vec<u32> = locations
        .iter()
        .map(|loc| search(loc, &instructions, &map))
        .collect();

    let lcm = lengths.iter().fold(1, |l, r| lcm(l as u64, *r as u64));
    return lcm;
}
