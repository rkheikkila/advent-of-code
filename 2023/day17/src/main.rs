use std::collections::{BinaryHeap, HashMap};

fn main() {
    let input = include_str!("../input.txt");
    day17(input);
}

fn min_heat_loss(
    matrix: &Vec<Vec<u32>>,
    start: (usize, usize),
    goal: (usize, usize),
    minstep: isize,
    maxstep: isize,
) -> i32 {
    let n_rows = matrix.len();
    let n_cols = matrix[0].len();
    let mut prev: HashMap<((usize, usize), (isize, isize)), ((usize, usize), (isize, isize))> =
        HashMap::new();
    let mut costs = HashMap::new();
    let mut visit_next = BinaryHeap::from_iter([(0, (start, (0, 0)))]);

    while let Some((cost, (pos, dir))) = visit_next.pop() {
        if pos == goal {
            return -cost;
        }

        if costs.get(&(pos, dir)).is_some_and(|&c| -cost > c) {
            continue;
        }

        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            if dir == (dx, dy) || dir == (-dx, -dy) {
                continue;
            }
            let mut new_cost = -cost;
            for dist in 1..=maxstep {
                let nx = (pos.0 as isize + dx * dist) as usize;
                let ny = (pos.1 as isize + dy * dist) as usize;
                if nx >= n_cols || ny >= n_rows {
                    continue;
                }
                new_cost += matrix[ny][nx] as i32;
                let key = ((nx, ny), (dx, dy));
                if dist >= minstep && new_cost < *costs.get(&key).unwrap_or(&1000000) {
                    prev.insert(key, (pos, dir));
                    costs.insert(key, new_cost);
                    visit_next.push((-new_cost, key));
                }
            }
        }
    }
    unreachable!()
}

fn day17(input: &str) {
    let matrix: Vec<Vec<u32>> = input
        .split("\n")
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let n_rows = matrix.len();
    let n_cols = matrix[0].len();

    let p1 = min_heat_loss(&matrix, (0, 0), (n_cols - 1, n_rows - 1), 1, 3);
    println!("{}", p1);

    let p2 = min_heat_loss(&matrix, (0, 0), (n_cols - 1, n_rows - 1), 4, 10);
    println!("{}", p2);
}
