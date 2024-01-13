use itertools::Itertools;
use std::io::BufRead;
use std::{fs::File as FSFile, io};

#[derive(Debug)]
struct File<'a> {
    path: Vec<&'a str>,
    name: &'a str,
    size: i32,
}

fn create_paths(path: Vec<&str>, size: i32) -> Vec<(Vec<&str>, i32)> {
    let mut result = Vec::new();
    for n in 0..path.len() + 1 {
        let head: Vec<&str> = path.iter().cloned().take(n).collect();
        result.push((head, size));
    }
    return result;
}

fn main() {
    let file = FSFile::open("input.txt").unwrap();
    let lines_iter = io::BufReader::new(file).lines();
    let lines = Vec::from_iter(lines_iter.map(|x| x.unwrap()));

    let mut files = Vec::new();
    let mut path = Vec::new();
    let mut pos = 1;

    while pos < lines.len() {
        println!("current dir {:?}", path.join("/"));
        let value = &lines[pos];
        if value.starts_with("$") {
            if value.starts_with("$ cd") {
                let dirname = &value[5..];
                if dirname == ".." {
                    path.pop();
                } else {
                    path.push(dirname);
                }
                pos += 1;
            } else if value.starts_with("$ ls") {
                pos += 1;
                while !lines[pos].starts_with("$") {
                    let line = &lines[pos];
                    if !line.starts_with("dir") {
                        let mut split = line.split_whitespace();
                        let size: i32 = split.next().unwrap().parse().unwrap();
                        let name = split.next().unwrap();
                        println!("adding file {:} from {:}", name, path.join("/"));
                        let new_file = File {
                            path: path.clone(),
                            name: name,
                            size: size,
                        };
                        files.push(new_file);
                    }
                    pos += 1;
                    if pos >= lines.len() {
                        break;
                    }
                }
            }
        }
    }
    //println!("{:?}", files);

    let groups = files
        .iter()
        .map(|f| (f.path.clone(), f.size))
        .flat_map(|(path, size)| create_paths(path, size))
        .map(|f| (f.0.join("/"), f.1))
        .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
        .group_by(|f| f.0.clone());

    let mut sizes = Vec::new();
    for (key, group) in &groups {
        let sum: i32 = group.map(|(_, y)| y).sum();
        sizes.push((key, sum));
    }
    println!("{:?}", sizes);

    let sum_part1: i32 = sizes.iter().filter(|s| s.1 <= 100000).map(|s| s.1).sum();
    println!("{:}", sum_part1);

    let total_size = 70000000;
    let free_size = total_size - sizes[0].1;
    let required_size = 30000000;
    let size_to_delete = required_size - free_size;

    sizes.sort_by(|a, b| Ord::cmp(&a.1, &b.1));
    println!("{:?}", sizes);
    let dirsize = sizes.iter().find(|f| f.1 > size_to_delete);
    println!("{:}", dirsize.unwrap().1);
}
