use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    part1(input);
    part2(input);
}

fn print_matrix(matrix: &Vec<Vec<char>>) {
    let n_rows = matrix.len();
    let n_cols = matrix[0].len();
    println!();
    for i in 0..n_rows {
        for j in 0..n_cols {
            print!("{}", matrix[i][j]);
        }
        println!();
    }
}

enum Direction {
    North,
    West,
    South,
    East,
}

fn part1(input: &str) {
    let mut matrix: Vec<Vec<char>> = input
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();

    let n_rows = matrix.len();
    let n_cols = matrix[0].len();

    for y in 1..n_rows {
        for x in 0..n_cols {
            if matrix[y][x] == 'O' {
                let mut curr_y = y;
                while curr_y > 0 && matrix[curr_y - 1][x] == '.' {
                    // swap
                    matrix[curr_y][x] = '.';
                    matrix[curr_y - 1][x] = 'O';
                    curr_y -= 1;
                }
            }
        }
    }

    let mut load = 0;
    for y in 0..n_rows {
        let mut row_load = 0;
        for x in 0..n_cols {
            let mut load = 0;
            if matrix[y][x] == 'O' {
                load = n_rows - y;
            }
            row_load += load;
        }
        load += row_load;
    }
    println!("{}", load);
}

fn apply_tilts(flat_matrix: &mut Vec<char>, n_cols: usize, n_rows: usize) {
    let directions = [
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ];
    for direction in &directions {
        match direction {
            Direction::North => {
                for y in 1..n_rows {
                    for x in 0..n_cols {
                        let pos = y * n_cols + x;
                        if flat_matrix[pos] == 'O' {
                            let mut curr_pos = pos;
                            while curr_pos >= n_cols && flat_matrix[curr_pos - n_cols] == '.' {
                                curr_pos -= n_cols;
                            }
                            // swap
                            flat_matrix[pos] = '.';
                            flat_matrix[curr_pos] = 'O';
                        }
                    }
                }
            }
            Direction::South => {
                for y in (0..n_rows).rev() {
                    for x in 0..n_cols {
                        let pos = y * n_cols + x;
                        if flat_matrix[pos] == 'O' {
                            let mut curr_pos = pos;
                            while curr_pos < flat_matrix.len() - n_cols
                                && flat_matrix[curr_pos + n_cols] == '.'
                            {
                                curr_pos += n_cols;
                            }
                            // swap
                            flat_matrix[pos] = '.';
                            flat_matrix[curr_pos] = 'O';
                        }
                    }
                }
            }
            Direction::West => {
                for y in 0..n_rows {
                    for x in 1..n_cols {
                        let pos = y * n_cols + x;
                        if flat_matrix[pos] == 'O' {
                            let mut curr_pos = pos;
                            let row_start = y * n_cols;
                            let row_end = (y + 1) * n_cols;
                            while curr_pos > row_start
                                && curr_pos < row_end
                                && flat_matrix[curr_pos - 1] == '.'
                            {
                                curr_pos -= 1;
                            }
                            // swap
                            flat_matrix[pos] = '.';
                            flat_matrix[curr_pos] = 'O';
                        }
                    }
                }
            }
            Direction::East => {
                for y in 0..n_rows {
                    for x in (0..n_cols).rev() {
                        let pos = y * n_cols + x;
                        if flat_matrix[pos] == 'O' {
                            let mut curr_pos = pos;
                            let row_start = y * n_cols;
                            let row_end = (y + 1) * n_cols;
                            while curr_pos >= row_start
                                && curr_pos < row_end - 1
                                && flat_matrix[curr_pos + 1] == '.'
                            {
                                curr_pos += 1;
                            }
                            // swap
                            flat_matrix[pos] = '.';
                            flat_matrix[curr_pos] = 'O';
                        }
                    }
                }
            }
        }
    }
}

fn part2(input: &str) {
    let matrix: Vec<Vec<char>> = input
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();

    print_matrix(&matrix);

    let n_rows = matrix.len();
    let n_cols = matrix[0].len();

    let mut flat_matrix: Vec<char> = matrix.into_iter().flatten().collect();

    let mut cache: HashMap<Vec<usize>, usize> = HashMap::new();

    let iterations = 1000000000;
    let mut remaining = iterations;

    for rounds in 0..iterations {
        let positions: Vec<usize> = flat_matrix
            .iter()
            .enumerate()
            .filter_map(|(i, &c)| if c == 'O' { Some(i) } else { None })
            .collect();

        if cache.contains_key(&positions) {
            //println!("Cache hit");
            let cycle_seen = cache.get(&positions).unwrap();
            let size = rounds - cycle_seen;
            remaining = ((iterations - rounds) as usize).rem_euclid(size);
            break;
        } else {
            apply_tilts(&mut flat_matrix, n_cols, n_rows);
            cache.insert(positions, rounds);
        }
        if (rounds as usize).rem_euclid(100000) == 0 {
            println!("{} rounds", rounds);
        }
    }

    for _ in 0..remaining {
        apply_tilts(&mut flat_matrix, n_cols, n_rows);
    }

    let reconstructed: Vec<Vec<char>> = flat_matrix.chunks(n_cols).map(|c| c.to_vec()).collect();

    print_matrix(&reconstructed);

    let mut load = 0;
    // count load on each row
    for y in 0..n_rows {
        let mut row_load = 0;
        for x in 0..n_cols {
            let mut load = 0;
            if reconstructed[y][x] == 'O' {
                load = n_rows - y;
            }
            row_load += load;
        }
        load += row_load;
    }
    println!("{}", load);
}
