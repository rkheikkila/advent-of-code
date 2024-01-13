use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    part1(input);
    part2(input);
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
enum Direction {
    North,
    South,
    West,
    East,
}

fn out_of_bounds(pos: (i32, i32), n_rows: i32, n_cols: i32) -> bool {
    if pos.0 < 0 || pos.0 >= n_cols || pos.1 < 0 || pos.1 >= n_rows {
        return true;
    } else {
        return false;
    }
}

fn traverse_grid(
    grid: &Vec<Vec<char>>,
    start: (i32, i32),
    init_direction: Direction,
    visited: &mut HashSet<((i32, i32), Direction)>,
) {
    let n_cols = grid[0].len() as i32;
    let n_rows = grid.len() as i32;
    let mut pos = start;
    let mut direction = init_direction;

    while !visited.contains(&(pos, direction.clone())) && !out_of_bounds(pos, n_rows, n_cols) {
        visited.insert((pos, direction.clone()));
        let char = grid[pos.1 as usize][pos.0 as usize];

        match (char, direction.clone()) {
            ('.', Direction::North) => pos = (pos.0, pos.1 - 1),
            ('.', Direction::South) => pos = (pos.0, pos.1 + 1),
            ('.', Direction::West) => pos = (pos.0 - 1, pos.1),
            ('.', Direction::East) => pos = (pos.0 + 1, pos.1),
            ('/', Direction::South) => {
                pos = (pos.0 - 1, pos.1);
                direction = Direction::West;
            }
            ('/', Direction::East) => {
                pos = (pos.0, pos.1 - 1);
                direction = Direction::North;
            }
            ('/', Direction::North) => {
                pos = (pos.0 + 1, pos.1);
                direction = Direction::East;
            }
            ('/', Direction::West) => {
                pos = (pos.0, pos.1 + 1);
                direction = Direction::South;
            }
            ('\\', Direction::South) => {
                pos = (pos.0 + 1, pos.1);
                direction = Direction::East;
            }
            ('\\', Direction::East) => {
                pos = (pos.0, pos.1 + 1);
                direction = Direction::South;
            }
            ('\\', Direction::North) => {
                pos = (pos.0 - 1, pos.1);
                direction = Direction::West;
            }
            ('\\', Direction::West) => {
                pos = (pos.0, pos.1 - 1);
                direction = Direction::North;
            }
            ('|', Direction::South) => pos = (pos.0, pos.1 + 1),
            ('|', Direction::North) => pos = (pos.0, pos.1 - 1),
            ('-', Direction::East) => pos = (pos.0 + 1, pos.1),
            ('-', Direction::West) => pos = (pos.0 - 1, pos.1),
            ('|', Direction::West) | ('|', Direction::East) => {
                traverse_grid(grid, (pos.0, pos.1 - 1), Direction::North, visited);
                traverse_grid(grid, (pos.0, pos.1 + 1), Direction::South, visited);
            }
            ('-', Direction::North) | ('-', Direction::South) => {
                traverse_grid(grid, (pos.0 - 1, pos.1), Direction::West, visited);
                traverse_grid(grid, (pos.0 + 1, pos.1), Direction::East, visited);
            }
            (_, _) => panic!("wtf"),
        }
    }
}

fn part1(input: &str) {
    let grid: Vec<Vec<char>> = input
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();

    let mut visited = HashSet::new();
    traverse_grid(&grid, (0, 0), Direction::East, &mut visited);

    let count = visited.iter().unique_by(|x| x.0).count();
    println!("{}", count);
}

fn part2(input: &str) {
    let grid: Vec<Vec<char>> = input
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();

    let n_rows = grid.len();
    let n_cols = grid[0].len();

    let mut best = 0;
    for i in 0..n_rows {
        let mut visited = HashSet::new();
        traverse_grid(&grid, (0, i as i32), Direction::East, &mut visited);
        let count = visited.iter().unique_by(|x| x.0).count();
        if count > best {
            best = count;
        }

        visited.clear();
        traverse_grid(
            &grid,
            ((n_cols - 1) as i32, i as i32),
            Direction::West,
            &mut visited,
        );
        let count = visited.iter().unique_by(|x| x.0).count();
        if count > best {
            best = count;
        }
    }

    for j in 0..n_cols {
        let mut visited = HashSet::new();
        traverse_grid(&grid, (j as i32, 0), Direction::South, &mut visited);
        let count = visited.iter().unique_by(|x| x.0).count();
        if count > best {
            best = count;
        }

        visited.clear();
        traverse_grid(
            &grid,
            (j as i32, (n_rows - 1) as i32),
            Direction::West,
            &mut visited,
        );
        let count = visited.iter().unique_by(|x| x.0).count();
        if count > best {
            best = count;
        }
    }

    println!("{}", best);
}
