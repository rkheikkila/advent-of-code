use std::collections::VecDeque;

fn main() {
    let input = include_str!("../input.txt");
    day10(input);
}

fn day10(input: &str) {
    let matrix: Vec<Vec<char>> = input
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();

    let mut start = (0, 0);
    for (y, row) in matrix.iter().enumerate() {
        if let Some(x) = row.iter().position(|&c| c == 'S') {
            start = (x, y);
        }
    }
    let mut path: Vec<(usize, usize)> = vec![start];
    let mut pos: (usize, usize) = (start.0, start.1 + 1);
    let mut last_move = (0, 1);

    while matrix[pos.1][pos.0] != 'S' {
        path.push(pos);
        let elem = matrix[pos.1][pos.0];
        match elem {
            '|' => {
                if last_move == (0, -1) {
                    pos.1 -= 1;
                } else if last_move == (0, 1) {
                    pos.1 += 1;
                }
            }
            '-' => {
                if last_move == (1, 0) {
                    pos.0 += 1;
                } else if last_move == (-1, 0) {
                    pos.0 -= 1;
                }
            }
            'L' => {
                if last_move == (-1, 0) {
                    pos.1 -= 1;
                    last_move = (0, -1);
                } else if last_move == (0, 1) {
                    pos.0 += 1;
                    last_move = (1, 0);
                }
            }
            'J' => {
                if last_move == (1, 0) {
                    pos.1 -= 1;
                    last_move = (0, -1);
                } else if last_move == (0, 1) {
                    pos.0 -= 1;
                    last_move = (-1, 0);
                }
            }
            'F' => {
                if last_move == (0, -1) {
                    pos.0 += 1;
                    last_move = (1, 0);
                } else if last_move == (-1, 0) {
                    pos.1 += 1;
                    last_move = (0, 1);
                }
            }
            '7' => {
                if last_move == (0, -1) {
                    pos.0 -= 1;
                    last_move = (-1, 0);
                } else if last_move == (1, 0) {
                    pos.1 += 1;
                    last_move = (0, 1);
                }
            }
            _ => panic!("oops"),
        }
    }

    let path_length = path.len() / 2;
    println!("{}", path_length);

    // Remove all pipe symbols except the ones that are part of the loop

    let mut new_matrix = matrix.clone();
    let size = new_matrix.len();
    for i in 0..size {
        for j in 0..size {
            if !path.contains(&(j, i)) {
                new_matrix[i][j] = '.';
            }
        }
    }
    let n_rows = new_matrix.len();
    let n_cols = new_matrix[0].len();

    // increase resolution by 3
    let mut super_matrix: Vec<Vec<char>> = Vec::new();
    for _ in 0..(3 * n_rows) {
        let row = vec!['.'; 3 * n_cols];
        super_matrix.push(row);
    }

    for y in 0..n_rows {
        for x in 0..n_cols {
            let elem = new_matrix[y][x];
            let (sx, sy) = (3 * x + 1, 3 * y + 1);
            match elem {
                '|' => {
                    super_matrix[sy][sx] = '|';
                    super_matrix[sy + 1][sx] = '|';
                    super_matrix[sy - 1][sx] = '|';
                }
                '-' => {
                    super_matrix[sy][sx] = '-';
                    super_matrix[sy][sx + 1] = '-';
                    super_matrix[sy][sx - 1] = '-';
                }
                'L' => {
                    super_matrix[sy][sx] = 'L';
                    super_matrix[sy - 1][sx] = 'L';
                    super_matrix[sy][sx + 1] = 'L';
                }
                'J' => {
                    super_matrix[sy][sx] = 'J';
                    super_matrix[sy - 1][sx] = 'J';
                    super_matrix[sy][sx - 1] = 'J';
                }
                'F' => {
                    super_matrix[sy][sx] = 'F';
                    super_matrix[sy + 1][sx] = 'F';
                    super_matrix[sy][sx + 1] = 'F';
                }
                '7' => {
                    super_matrix[sy][sx] = '7';
                    super_matrix[sy + 1][sx] = '7';
                    super_matrix[sy][sx - 1] = '7';
                }
                'S' => {
                    super_matrix[sy][sx] = 'S';
                    super_matrix[sy + 1][sx] = 'S';
                    super_matrix[sy - 1][sx] = 'S';
                    super_matrix[sy][sx + 1] = 'S';
                    super_matrix[sy][sx - 1] = 'S';
                }
                _ => (),
            }
        }
    }

    // Flood fill to find all positions that can be reached from outside of the loop

    let n_rows_super = super_matrix.len();
    let n_cols_super = super_matrix[0].len();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    for y in 0..n_rows_super {
        for x in 0..n_cols_super {
            let on_edge =
                (x == 0) || (y == 0) || (x == n_cols_super - 1) || (y == n_rows_super - 1);
            if on_edge && super_matrix[y][x] == '.' {
                super_matrix[y][x] = 'V';
                queue.push_back((x, y));
            }
        }
    }

    let deltas = vec![
        (-1, -1),
        (-1, 0),
        (0, -1),
        (-1, 1),
        (-1, 1),
        (1, 0),
        (0, 1),
        (1, 1),
    ];

    while let Some(node) = queue.pop_front() {
        for (dx, dy) in deltas.iter() {
            let new_x = node.0 as i32 + dx;
            let new_y = node.1 as i32 + dy;
            let within_bounds = (new_x >= 0)
                && (new_y >= 0)
                && (new_x < n_cols_super as i32)
                && (new_y < n_rows_super as i32);
            if within_bounds {
                let (nx, ny) = (new_x as usize, new_y as usize);
                if super_matrix[ny][nx] == '.' {
                    super_matrix[ny][nx] = 'V';
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    // Visualize end state

    println!();
    for y in 0..n_rows {
        for x in 0..n_cols {
            let (sx, sy) = (3 * x + 1, 3 * y + 1);
            print!("{}", super_matrix[sy][sx]);
        }
        println!();
    }

    let mut area = 0;
    for y in 0..n_rows {
        for x in 0..n_cols {
            let (sx, sy) = (3 * x + 1, 3 * y + 1);
            let elem = super_matrix[sy][sx];
            if elem == '.' {
                area += 1;
            }
        }
    }
    println!("{}", area);
}
