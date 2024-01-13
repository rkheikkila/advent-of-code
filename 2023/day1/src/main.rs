use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let answer1 = part1(input);
    let answer2 = part2(input);
    println!("{}", answer1);
    println!("{}", answer2);
}

fn part1(input: &str) -> i32 {
    let lines = input.split("\n");
    let sum: i32 = lines.map(|s| find_digits_part1(s)).sum();
    return sum;
}

fn find_digits_part1(s: &str) -> i32 {
    let mut leftmost: i32 = -1;
    let mut rightmost: i32 = -1;
    for c in s.chars() {
        if c.is_numeric() {
            let int_value = c.to_string().parse::<i32>().unwrap();
            if leftmost == -1 {
                leftmost = int_value;
            }
            rightmost = int_value;
        }
    }
    return leftmost * 10 + rightmost;
}

fn part2(input: &str) -> i32 {
    let lines = input.split("\n");
    let sum: i32 = lines.map(|s| find_digits_part2(s)).sum();
    return sum;
}

fn find_digits_part2(s: &str) -> i32 {
    let numbers: HashMap<&str, i32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut leftmost: i32 = -1;
    let mut rightmost: i32 = -1;

    for i in 0..s.len() {
        let substr = &s[i..];
        if let Some(c) = substr.chars().next() {
            if c.is_numeric() {
                leftmost = c.to_string().parse::<i32>().unwrap();
                break;
            }
        }
        if let Some(name) = numbers.keys().find(|&n| substr.starts_with(n)) {
            leftmost = numbers.get(name).unwrap().clone();
            break;
        }
    }

    for i in (0..=s.len()).rev() {
        let substr = &s[0..i];
        if let Some(c) = substr.chars().rev().next() {
            if c.is_numeric() {
                rightmost = c.to_string().parse::<i32>().unwrap();
                break;
            }
        }
        if let Some(name) = numbers.keys().find(|&n| substr.ends_with(n)) {
            rightmost = numbers.get(name).unwrap().clone();
            break;
        }
    }

    println!("{}", s);
    println!("{}, {}", leftmost, rightmost);
    return leftmost * 10 + rightmost;
}
