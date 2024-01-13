fn main() {
    let input = include_str!("../input.txt");
    let answer1 = part1(input);
    println!("{}", answer1);
    let answer2 = part2(input);
    println!("{}", answer2);
}

struct IntervalMap {
    start: u64,
    end: u64,
    diff: i64,
}

fn part1(input: &str) -> u64 {
    let lines: Vec<&str> = input.split("\n").collect();

    let seeds: Vec<u64> = lines
        .iter()
        .nth(0)
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let mut indices = seeds.clone();

    let map_positions = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains("map"))
        .map(|(i, _)| i);

    for pos in map_positions {
        let map_lines = lines
            .iter()
            .skip(pos + 1)
            .take_while(|line| !line.trim().is_empty());

        let mut map: Vec<IntervalMap> = Vec::new();
        for line in map_lines {
            let (start, tail) = line.split_once(" ").unwrap();
            let (end, range) = tail.split_once(" ").unwrap();

            let start_num = start.parse::<u64>().unwrap();
            let end_num = end.parse::<u64>().unwrap();
            let range_num = range.parse::<u64>().unwrap();

            map.push(IntervalMap {
                start: end_num,
                end: end_num + range_num,
                diff: (start_num as i64) - (end_num as i64),
            })
        }

        for i in 0..indices.len() {
            let key = indices[i];
            for im in map.iter() {
                if (key >= im.start) && (key < im.end) {
                    let new_key = (indices[i] as i64 + im.diff) as u64;
                    println!("key {} corresponds to {}", key, new_key);
                    indices[i] = new_key;
                    break;
                }
            }
        }
    }
    return *indices.iter().min().unwrap();
}

fn part2(input: &str) -> u64 {
    let lines: Vec<&str> = input.split("\n").collect();

    let seeds: Vec<u64> = lines
        .iter()
        .nth(0)
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let seed_starts = seeds.iter().step_by(2);
    let seed_ranges = seeds.iter().skip(1).step_by(2);

    let mut seeds: Vec<u64> = Vec::new();
    for (start, range) in seed_starts.zip(seed_ranges) {
        for i in 0..*range {
            seeds.push(start + i);
        }
    }

    let mut indices = seeds.clone();

    let map_positions = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains("map"))
        .map(|(i, _)| i);

    for pos in map_positions {
        let map_lines = lines
            .iter()
            .skip(pos + 1)
            .take_while(|line| !line.trim().is_empty());

        let mut map: Vec<IntervalMap> = Vec::new();
        for line in map_lines {
            let (start, tail) = line.split_once(" ").unwrap();
            let (end, range) = tail.split_once(" ").unwrap();

            let start_num = start.parse::<u64>().unwrap();
            let end_num = end.parse::<u64>().unwrap();
            let range_num = range.parse::<u64>().unwrap();

            map.push(IntervalMap {
                start: end_num,
                end: end_num + range_num,
                diff: (start_num as i64) - (end_num as i64),
            })
        }

        for i in 0..indices.len() {
            let key = indices[i];
            for im in map.iter() {
                if (key >= im.start) && (key < im.end) {
                    let new_key = (indices[i] as i64 + im.diff) as u64;
                    indices[i] = new_key;
                    break;
                }
            }
        }
    }
    return *indices.iter().min().unwrap();
}
