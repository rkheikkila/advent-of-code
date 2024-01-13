fn main() {
    let input = include_str!("../input.txt");
    let answer1 = part1(input);
    println!("{}", answer1);
    let answer2 = part2(input);
    println!("{}", answer2);
}

#[derive(Debug)]
struct Record {
    time: u64,
    distance: u64,
}

// -x^2 + tx - d >= 0

fn count_ways_to_win(record: &Record) -> u64 {
    let time = record.time;
    let winning_distance = record.distance;
    let discriminant = f64::sqrt((time.pow(2) - 4 * winning_distance) as f64);
    let point1 = -0.5 * (time as f64 + term);
    let point2 = -0.5 * (time as f64 - term);

    let point1_int = point1.floor() as i64;
    let point2_int = point2.floor() as i64;
    let count = (point2_int - point1_int) as u64;
    return count;
}

fn part1(input: &str) -> u64 {
    let lines = input
        .split("\n")
        .map(|line| line.split_once(":").unwrap().1);
    let times: Vec<u32> = lines
        .clone()
        .nth(0)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let distances: Vec<u32> = lines
        .clone()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut records: Vec<Record> = Vec::new();
    for (time, distance) in times.iter().zip(distances.iter()) {
        let record = Record {
            time: *time as u64,
            distance: *distance as u64,
        };
        records.push(record)
    }
    let counts: u64 = records.iter().map(|r| count_ways_to_win(r)).product();

    return counts;
}

fn part2(input: &str) -> u64 {
    let lines = input
        .split("\n")
        .map(|line| line.split_once(":").unwrap().1);
    let time: String = lines
        .clone()
        .nth(0)
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();
    let distance: String = lines
        .clone()
        .nth(1)
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();

    let time_num = time.parse::<u64>().unwrap();
    let distance_num = distance.parse::<u64>().unwrap();
    let record = Record {
        time: time_num,
        distance: distance_num,
    };
    return count_ways_to_win(&record);
}
