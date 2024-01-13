use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input.txt").unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut counts = vec![0];
    let mut index = 0;
    for line in lines {
        if let Ok(value) = line {
            match value.parse::<i32>() {
                Ok(v) => {
                    counts[index] += v;
                }
                Err(_) => {
                    counts.push(0);
                    index += 1;
                }
            }
        }
    }

    counts.sort();
    counts.reverse();

    let max_value = counts[0];
    println!("{:}", max_value);

    let top3_total: i32 = counts[0..3].iter().sum();
    println!("{:}", top3_total);
}
