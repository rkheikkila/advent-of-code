use itertools::peek_nth;
use itertools::Itertools;
use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input.txt").unwrap();
    let lines_iter = io::BufReader::new(file).lines();
    let lines = Vec::from_iter(lines_iter.map(|x| x.unwrap()));

    for line in lines.iter() {
        let mut chars_iter = peek_nth(line.chars());
        let mut pos = 0;
        while let Some(c) = chars_iter.next() {
            let mut temp = vec![c];
            for i in 0..3 {
                if let Some(k) = chars_iter.peek_nth(i) {
                    temp.push(k.clone());
                }
            }
            if temp.iter().unique().count() == 4 {
                println!("{:?}", temp);
                break;
            } else {
                pos += 1
            }
        }
        println!("{:}", pos + 4)
    }

    for line in lines.iter() {
        let mut chars_iter = peek_nth(line.chars());
        let mut pos = 0;
        while let Some(c) = chars_iter.next() {
            let mut temp = vec![c];
            for i in 0..13 {
                if let Some(k) = chars_iter.peek_nth(i) {
                    temp.push(k.clone());
                }
            }
            if temp.iter().unique().count() == 14 {
                println!("{:?}", temp);
                break;
            } else {
                pos += 1
            }
        }
        println!("{:}", pos + 14)
    }
}
