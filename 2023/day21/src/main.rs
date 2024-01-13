use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let steps1 = 64;
    let steps2 = 500;
    part1(input, steps1);
    part2(input, steps2);
}

fn part1(input: &str, steps: usize) {
    let matrix: Vec<Vec<char>> = input
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();

    let start_pos = matrix
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter()
                .position(|&c| c == 'S')
                .map(|col_idx| (row_idx, col_idx))
        })
        .next()
        .unwrap();

    let mut positions = Vec::from_iter([start_pos]);
    for _ in 0..steps {
        let new_pos = find_next_steps(&matrix, &positions);
        positions = new_pos.into_iter().unique().collect();
    }
    println!("{}", positions.len());
}

fn find_next_steps(
    matrix: &Vec<Vec<char>>,
    positions: &Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut new_positions: Vec<(usize, usize)> = Vec::new();

    let nrows = matrix.len();
    let ncols = matrix[0].len();

    let deltas: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    for pos in positions {
        for (dx, dy) in deltas {
            let nx = pos.0 as i32 + dx;
            let ny = pos.1 as i32 + dy;
            if nx >= 0 && ny >= 0 && ny < nrows as i32 && nx < ncols as i32 {
                if matrix[ny as usize][nx as usize] != '#' {
                    let new_pos = (nx as usize, ny as usize);
                    new_positions.push(new_pos)
                }
            }
        }
    }
    return new_positions;
}

fn find_steps(matrix: &Vec<Vec<char>>, pos: (isize, isize)) -> Vec<(isize, isize)> {
    let mut new_positions: Vec<(isize, isize)> = Vec::new();

    let nrows = matrix.len() as isize;
    let ncols = matrix[0].len() as isize;

    let deltas: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    for (dx, dy) in deltas {
        let nx = pos.0 as isize + dx;
        let ny = pos.1 as isize + dy;

        let rx = nx.rem_euclid(ncols);
        let ry = ny.rem_euclid(nrows);
        {
            if matrix[ry as usize][rx as usize] != '#' {
                let new_pos = (nx as isize, ny as isize);
                new_positions.push(new_pos)
            }
        }
    }
    return new_positions;
}

fn compute_steps(
    matrix: &Vec<Vec<char>>,
    start: &Vec<(isize, isize)>,
    steps: usize,
) -> Vec<(isize, isize)> {
    let mut positions = start.clone();
    for _ in 0..steps {
        let new_pos: Vec<(isize, isize)> = positions
            .iter()
            .flat_map(|&x| find_steps(&matrix, x))
            .unique()
            .collect();
        positions = new_pos;
    }
    return positions;
}

fn part2(input: &str, steps: usize) {
    let matrix: Vec<Vec<char>> = input
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();

    let start_pos = matrix
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter()
                .position(|&c| c == 'S')
                .map(|col_idx| (row_idx as isize, col_idx as isize))
        })
        .next()
        .unwrap();

    let start = Vec::from_iter([start_pos]);

    let positions0 = compute_steps(&matrix, &start, 65);
    let positions1 = compute_steps(&matrix, &start, 65 + 131);
    let positions2 = compute_steps(&matrix, &start, 65 + 2 * 131);

    let y0 = positions0.len() as isize;
    let y1 = positions1.len() as isize;
    let y2 = positions2.len() as isize;

    println!("x={}, y={}", 0, y0);
    println!("x={}, y={}", 1, y1);
    println!("x={}, y={}", 2, y2);

    let a = (y0 - 2 * y1 + y2) / 2;
    let b = (4 * y1 - 3 * y0 - y2) / 2;
    let c = y0;

    let x = (26501365 - 65) / 131;
    let count = a * x * x + b * x + c;
    println!("{}", count);
}
