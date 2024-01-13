use std::collections::BTreeMap;
fn main() {
    let input = include_str!("../input.txt");
    let answer1 = part1(input);
    println!("{}", answer1);
    let answer2 = part2(input);
    println!("{}", answer2);
}

fn check_adjacency(chars: &Vec<Vec<char>>, x: u32, y: u32) -> bool {
    let moves = [-1, 0, 1];
    for x_move in moves {
        for y_move in moves {
            let x_pos = i64::from(x) + x_move;
            let y_pos = i64::from(y) + y_move;

            // check bounds
            let n_rows = chars.len() as i64;
            let n_cols = chars[0].len() as i64;

            let valid_pos = (0 <= x_pos) && (x_pos < n_cols) && (0 <= y_pos) && (y_pos < n_rows);
            if valid_pos {
                let char = chars[y_pos as usize][x_pos as usize];
                if char != '.' && !char.is_numeric() {
                    return true;
                }
            }
        }
    }
    return false;
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;
    let lines = input.split("\n");
    let n_cols = lines.clone().next().expect("should exist").len();
    let n_rows = lines.clone().count();
    let chars: Vec<Vec<char>> = lines.map(|s| s.chars().collect()).collect();

    for i in 0..n_rows {
        let mut current_number_sequence: Vec<char> = Vec::new();
        let mut is_valid = false;

        for j in 0..n_cols {
            let char = chars[i][j];
            if char.is_numeric() {
                let res = check_adjacency(&chars, j as u32, i as u32);
                is_valid = res || is_valid;
                current_number_sequence.push(char);
            }
            if !char.is_numeric() || (j == n_cols - 1) {
                if !current_number_sequence.is_empty() {
                    let number_str = String::from_iter(current_number_sequence.iter());

                    if is_valid {
                        sum += number_str.parse::<u32>().expect("should parse");
                    } else {
                    }
                }
                current_number_sequence.clear();
                is_valid = false;
            }
        }
    }

    return sum;
}

fn find_gear(chars: &Vec<Vec<char>>, x: u32, y: u32) -> Option<(usize, usize)> {
    let moves = [-1, 0, 1];
    for x_move in moves {
        for y_move in moves {
            let x_pos = i64::from(x) + x_move;
            let y_pos = i64::from(y) + y_move;

            // check bounds
            let n_rows = chars.len() as i64;
            let n_cols = chars[0].len() as i64;

            let valid_pos = (0 <= x_pos) && (x_pos < n_cols) && (0 <= y_pos) && (y_pos < n_rows);
            if valid_pos {
                let char = chars[y_pos as usize][x_pos as usize];
                if char == '*' {
                    return Some((x_pos as usize, y_pos as usize));
                }
            }
        }
    }
    return None;
}

fn part2(input: &str) -> u32 {
    let lines = input.split("\n");
    let n_cols = lines.clone().next().expect("should exist").len();
    let n_rows = lines.clone().count();
    let chars: Vec<Vec<char>> = lines.map(|s| s.chars().collect()).collect();

    let mut res: BTreeMap<(usize, usize), Vec<u32>> = BTreeMap::new();

    for i in 0..n_rows {
        let mut current_number_sequence: Vec<char> = Vec::new();
        let mut gear_pos = None;

        for j in 0..n_cols {
            let char = chars[i][j];
            if char.is_numeric() {
                let pos = find_gear(&chars, j as u32, i as u32);
                if gear_pos.is_none() {
                    gear_pos = pos;
                }
                current_number_sequence.push(char);
            }
            if !char.is_numeric() || (j == n_cols - 1) {
                if !current_number_sequence.is_empty() {
                    let number_str = String::from_iter(current_number_sequence.iter());

                    match gear_pos {
                        Some((x, y)) => {
                            let num = number_str.parse::<u32>().expect("should parse");
                            let key = res.get(&(x, y));
                            match key {
                                Some(e) => {
                                    res.get_mut(&(x, y)).unwrap().push(num);
                                }
                                None => {
                                    let mut vec = Vec::new();
                                    vec.push(num);
                                    res.insert((x, y), vec);
                                    ();
                                }
                            }
                        }
                        None => (),
                    }
                }
                current_number_sequence.clear();
                gear_pos = None;
            }
        }
    }
    let mut sum = 0;
    for (k, v) in res.iter() {
        println!("Numbers {:?}", v);
        if v.len() > 1 {
            sum += v.iter().fold(1, |acc, &x| acc * x);
        }
    }
    return sum;
}
