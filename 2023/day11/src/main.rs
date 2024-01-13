use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    let answer1 = day11(input, 1);
    println!("{}", answer1);

    let answer2 = day11(input, 1000000 - 1);
    println!("{}", answer2);
}

fn day11(input: &str, gap_size: usize) -> usize {
    let matrix: Vec<Vec<char>> = input
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();

    let n_rows = matrix.len();
    let n_cols = matrix[0].len();
    let row_positions: Vec<(usize, usize)> = (&matrix)
        .iter()
        .enumerate()
        .filter_map(|(i, row)| {
            if row.iter().all(|&c| c == '.') {
                return Some((i, gap_size));
            } else {
                return None;
            }
        })
        .collect();

    let mut col_positions: Vec<(usize, usize)> = Vec::new();

    for i in 0..n_cols {
        let mut expand = true;
        for j in 0..n_rows {
            if matrix[j][i] != '.' {
                expand = false;
            }
        }
        if expand {
            col_positions.push((i, gap_size));
        }
    }

    let galaxy_pos: Vec<(usize, usize)> = matrix
        .iter()
        .enumerate()
        .flat_map(|(j, row)| {
            row.iter().enumerate().filter_map(move |(i, &c)| {
                if c == '#' {
                    return Some((i, j));
                } else {
                    return None;
                }
            })
        })
        .collect();

    let distance_sum: i64 = galaxy_pos
        .iter()
        .combinations(2)
        .map(|v| {
            let (g1, g2) = (v[0], v[1]);
            let col_gaps: usize = col_positions
                .iter()
                .filter_map(|(c, gap)| {
                    if ((*c > g1.0) && (*c < g2.0)) || ((*c < g1.0) && (*c > g2.0)) {
                        return Some(gap);
                    } else {
                        return None;
                    }
                })
                .sum();
            let row_gaps: usize = row_positions
                .iter()
                .filter_map(|(r, gap)| {
                    if ((*r > g1.1) && (*r < g2.1)) || ((*r < g1.1) && (*r > g2.1)) {
                        return Some(gap);
                    } else {
                        return None;
                    }
                })
                .sum();
            return (g1.0 as i64 - g2.0 as i64).abs()
                + col_gaps as i64
                + (g1.1 as i64 - g2.1 as i64).abs()
                + row_gaps as i64;
        })
        .sum();
    return distance_sum as usize;
}
