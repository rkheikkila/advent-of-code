use ndarray::s;
use ndarray::Array;
use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = BufReader::new(File::open("input.txt").unwrap());

    let v: Vec<Vec<u32>> = f
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|number| number.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let ys = v.len();
    let xs = v[0].len();

    let arr = Array::from_shape_vec((ys, xs), v.iter().flat_map(|x| x).collect()).unwrap();

    let mut visible = Array::<i32, _>::zeros(arr.raw_dim());
    for i in 0..ys {
        for j in 0..xs {
            if i == 0 || j == 0 || i == ys - 1 || j == ys - 1 {
                visible[(i, j)] = 1;
            } else {
                let height = arr[(i, j)];
                // check top
                let top_height = arr.slice(s![0..i, j]).iter().max().unwrap().clone();
                if height > top_height {
                    visible[(i, j)] = 1;
                    continue;
                }
                let left_height = arr.slice(s![i, 0..j;-1]).iter().max().unwrap().clone();
                if height > left_height {
                    visible[(i, j)] = 1;
                    continue;
                }
                let bottom_height = arr.slice(s![i + 1..ys, j]).iter().max().unwrap().clone();
                if height > bottom_height {
                    visible[(i, j)] = 1;
                    continue;
                }
                let right_height = arr.slice(s![i, j + 1..xs]).iter().max().unwrap().clone();
                if height > right_height {
                    visible[(i, j)] = 1;
                    continue;
                }
            }
        }
    }
    //println!("{:?}", arr.slice(s![97..99, 95]));
    println!("{:?}", visible);
    let total_visible: i32 = visible.iter().sum();
    println!("{:?}", total_visible);

    let mut score = Array::<i32, _>::zeros(arr.raw_dim());
    for i in 0..ys {
        for j in 0..xs {
            if i == 0 || j == 0 || i == ys - 1 || j == ys - 1 {
                continue;
            } else {
                let height = arr[(i, j)];
                // check top
                let mut top_score: i32 = 0;
                for k in (0..i).rev() {
                    if height > arr[(k, j)] {
                        top_score += 1;
                    } else {
                        top_score += 1;
                        break;
                    }
                }

                let mut left_score: i32 = 0;
                for k in (0..j).rev() {
                    if height > arr[(i, k)] {
                        left_score += 1;
                    } else {
                        left_score += 1;
                        break;
                    }
                }

                let mut bottom_score: i32 = 0;
                for k in (i + 1..ys) {
                    if height > arr[(k, j)] {
                        bottom_score += 1;
                    } else {
                        bottom_score += 1;
                        break;
                    }
                }

                let mut right_score: i32 = 0;
                for k in (j + 1..xs) {
                    if height > arr[(i, k)] {
                        right_score += 1;
                    } else {
                        right_score += 1;
                        break;
                    }
                }

                score[(i, j)] = top_score * bottom_score * left_score * right_score;
            }
        }
    }
    println!("{:?}", score);
    println!("{:}", score.iter().max().unwrap());
}
