use std::collections::{BinaryHeap, HashSet};

fn main() {
    let input = include_str!("../input.txt");
    day23(input);
}

const DELTAS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn day23(input: &str) {
    let grid = input.split("\n").map(str::as_bytes).collect::<Vec<_>>();

    let start_x = grid[0].iter().position(|&c| c == b'.').unwrap();
    let start_pos = (start_x, 0);

    let end_x = grid
        .last()
        .unwrap()
        .iter()
        .position(|&c| c == b'.')
        .unwrap();
    let end_pos = (end_x, grid.len() - 1);

    let path_length1 = find_longest_path(&grid, &start_pos, &end_pos, &get_neighbors_slopes);
    println!("{}", path_length1);

    let path_length2 = find_longest_path(&grid, &start_pos, &end_pos, &get_neighbors);
    println!("{}", path_length2);
}

fn get_neighbors(matrix: &[&[u8]], pos: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    for (dx, dy) in &DELTAS {
        let nx = pos.0 as i32 + dx;
        let ny = pos.1 as i32 + dy;
        if nx >= 0 && ny >= 0 && nx < matrix[0].len() as i32 && ny < matrix.len() as i32 {
            let npos = (nx as usize, ny as usize);
            let c = matrix[npos.1][npos.0];
            if c != b'#' {
                neighbors.push(npos);
            }
        }
    }
    return neighbors;
}

fn get_neighbors_slopes(matrix: &[&[u8]], pos: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    let c = matrix[pos.1][pos.0];
    if c == b'v' {
        neighbors = vec![(pos.0, pos.1 + 1)];
    } else if c == b'>' {
        neighbors = vec![(pos.0 + 1, pos.1)];
    } else {
        for (dx, dy) in &DELTAS {
            let nx = pos.0 as i32 + dx;
            let ny = pos.1 as i32 + dy;
            if nx >= 0 && ny >= 0 && nx < matrix[0].len() as i32 && ny < matrix.len() as i32 {
                let npos = (nx as usize, ny as usize);
                let c = matrix[npos.1][npos.0];
                if c == b'.' || (c == b'v' && pos.1 < npos.1) || (c == b'>' && pos.0 < npos.0) {
                    neighbors.push(npos);
                }
            }
        }
    }
    return neighbors;
}

fn find_longest_path(
    matrix: &[&[u8]],
    start_pos: &(usize, usize),
    end_pos: &(usize, usize),
    get_neighbors: &dyn Fn(&[&[u8]], &(usize, usize)) -> Vec<(usize, usize)>,
) -> usize {
    let mut visited = HashSet::new();
    let mut current_path = Vec::new();
    let mut path_lengths: BinaryHeap<usize> = BinaryHeap::new();

    dfs(
        matrix,
        start_pos,
        end_pos,
        get_neighbors,
        &mut visited,
        &mut current_path,
        &mut path_lengths,
    );

    let longest_path = path_lengths.peek().unwrap();
    return *longest_path;
}

fn dfs(
    matrix: &[&[u8]],
    current_pos: &(usize, usize),
    end_pos: &(usize, usize),
    get_neighbors: &dyn Fn(&[&[u8]], &(usize, usize)) -> Vec<(usize, usize)>,
    visited: &mut HashSet<(usize, usize)>,
    current_path: &mut Vec<(usize, usize)>,
    path_lengths: &mut BinaryHeap<usize>,
) {
    visited.insert(*current_pos);
    current_path.push(*current_pos);

    if current_pos == end_pos {
        let path_len = current_path.len() - 1;
        path_lengths.push(path_len);
        println!("Best so far: {} ", path_lengths.peek().unwrap_or(&0));
    } else {
        let neighbors = get_neighbors(matrix, current_pos);
        for neighbor in neighbors.iter() {
            if !visited.contains(neighbor) {
                dfs(
                    matrix,
                    neighbor,
                    end_pos,
                    get_neighbors,
                    visited,
                    current_path,
                    path_lengths,
                );
            }
        }
    }
    visited.remove(current_pos);
    current_path.pop();
}
