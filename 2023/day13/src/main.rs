fn main() {
    let input = include_str!("../input.txt");
    part1(input);
    part2(input);
}

fn find_reflection(matrix: &Vec<Vec<char>>, discard_c: Option<usize>) -> Option<usize> {
    let n_cols = matrix[0].len();
    let n_rows = matrix.len();

    for c in 1..n_cols {
        let mut found = true;
        for r in 0..n_rows {
            let row = &matrix[r];
            found = row[0..c]
                .iter()
                .rev()
                .zip(row[c..n_cols].iter())
                .all(|(left, right)| left == right);
            if !found {
                break;
            }
        }
        if found {
            if discard_c.is_some() {
                if discard_c.unwrap() != c {
                    return Some(c);
                }
            } else {
                return Some(c);
            }
        }
    }
    return None;
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn part1(input: &str) {
    let values: Vec<usize> = input
        .split("\n\n")
        .map(|lines| {
            lines
                .split("\n")
                .map(|line| line.chars().collect())
                .collect::<Vec<Vec<char>>>()
        })
        .map(|matrix| {
            let col_reflect: usize = find_reflection(&matrix, None).unwrap_or(0);
            let row_reflect: usize = find_reflection(&transpose(matrix), None).unwrap_or(0);
            col_reflect + 100 * row_reflect
        })
        .collect();
    println!("{}", values.iter().sum::<usize>());
}

fn part2(input: &str) {
    let values: Vec<usize> = input
        .split("\n\n")
        .map(|lines| {
            lines
                .split("\n")
                .map(|line| line.chars().collect())
                .collect::<Vec<Vec<char>>>()
        })
        .map(|matrix| -> usize {
            let orig_col_reflect: usize = find_reflection(&matrix, None).unwrap_or(0);
            let orig_row_reflect: usize =
                find_reflection(&transpose(matrix.clone()), None).unwrap_or(0);
            let mut col_reflect = 0;
            let mut row_reflect = 0;
            let n_cols = matrix[0].len();
            let n_rows = matrix.len();

            'outer: for j in 0..n_rows {
                for i in 0..n_cols {
                    let mut clone = matrix.clone();
                    if clone[j][i] == '.' {
                        clone[j][i] = '#';
                    } else {
                        clone[j][i] = '.';
                    }

                    col_reflect = find_reflection(&clone, Some(orig_col_reflect)).unwrap_or(0);
                    row_reflect =
                        find_reflection(&transpose(clone.clone()), Some(orig_row_reflect))
                            .unwrap_or(0);

                    if ((col_reflect > 0) && (col_reflect != orig_col_reflect)
                        || (row_reflect > 0) && (row_reflect != orig_row_reflect))
                    {
                        break 'outer;
                    }
                }
            }

            if (row_reflect > 0) && (row_reflect != orig_row_reflect) {
                100 * row_reflect
            } else {
                col_reflect
            }
        })
        .collect();
    println!("{}", values.iter().sum::<usize>());
}
