use itertools::Itertools;
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input.txt").unwrap();
    let lines_iter = io::BufReader::new(file).lines();
    let lines = Vec::from_iter(lines_iter.map(|x| x.unwrap()));

    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut priorities: HashMap<char, usize> = HashMap::new();
    for (i, char) in chars.chars().enumerate() {
        priorities.insert(char.clone(), i + 1);
    }

    let mut total_priority = 0;

    for line in lines.iter() {
        let length = line.len();
        let midpoint = length / 2;
        let first_half = &line[0..midpoint];
        let second_half = &line[midpoint..];

        let first_half_chars: HashSet<char> = HashSet::from_iter(first_half.chars());
        let second_half_chars: HashSet<char> = HashSet::from_iter(second_half.chars());
        for char in first_half_chars.intersection(&second_half_chars) {
            let priority = priorities.get(char).unwrap();
            total_priority += priority;
        }
    }
    println!("{:}", total_priority);

    // part 2
    let mut total_priority_part2: usize = lines
        .chunks(3)
        .flat_map(|elems| {
            elems[0]
                .chars()
                .find(|&c| elems[1].contains(c) && elems[2].contains(c))
        })
        .flat_map(|x| priorities.get(&x))
        .sum();

    println!("{:}", total_priority_part2);
}
