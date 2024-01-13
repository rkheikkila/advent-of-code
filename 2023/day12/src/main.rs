use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let a1 = part1(input);
    println!("{}", a1);
    let a2 = part2(input);
    println!("{}", a2);
}

fn search(
    cache: &mut HashMap<(usize, usize, u32), usize>,
    seq: &str,
    counts: &[u32],
    bad_count: Option<u32>,
) -> usize {
    if seq.is_empty() {
        return match (bad_count, counts.len()) {
            (None, 0) => 1,
            (Some(x), 1) if x == counts[0] => 1,
            _ => 0,
        };
    }

    if bad_count.is_some() && counts.is_empty() {
        return 0;
    }

    let key = (seq.len(), counts.len(), bad_count.unwrap_or(0));
    if let Some(&x) = cache.get(&key) {
        return x;
    }

    let count = match (&seq.chars().nth(0).unwrap(), bad_count) {
        ('.', Some(x)) if x != counts[0] => 0,
        ('.', Some(_)) => search(cache, &seq[1..], &counts[1..], None),
        ('.', None) => search(cache, &seq[1..], counts, None),
        ('#', None) => search(cache, &seq[1..], counts, Some(1)),
        ('#', Some(x)) => search(cache, &seq[1..], counts, Some(x + 1)),
        ('?', Some(x)) => {
            let mut c = search(cache, &seq[1..], counts, Some(x + 1));
            if x == counts[0] {
                c += search(cache, &seq[1..], &counts[1..], None);
            }
            c
        }
        ('?', None) => {
            search(cache, &seq[1..], counts, Some(1)) + search(cache, &seq[1..], counts, None)
        }
        _ => panic!(),
    };
    cache.insert(key, count);
    count
}

fn part1(input: &str) -> usize {
    let mut cache: HashMap<(usize, usize, u32), usize> = HashMap::new();
    input
        .split("\n")
        .map(|line| line.split_once(" ").unwrap())
        .map(|(seq, counts)| {
            let c = counts
                .split(",")
                .map(|c| c.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            return (seq, c);
        })
        .map(|(seq, counts)| {
            cache.clear();
            search(&mut cache, seq, &counts, None)
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let mut cache: HashMap<(usize, usize, u32), usize> = HashMap::new();
    input
        .split("\n")
        .map(|line| line.split_once(" ").unwrap())
        .map(|(seq, counts)| {
            let s = (0..5).map(|_| seq).collect::<Vec<&str>>().join("?");
            let c_iter = counts.split(",").map(|c| c.parse::<u32>().unwrap());
            let c = std::iter::repeat(c_iter)
                .take(5)
                .flatten()
                .collect::<Vec<u32>>();
            return (s, c);
        })
        .map(|(seq, counts)| {
            cache.clear();
            search(&mut cache, seq.as_str(), &counts, None)
        })
        .sum()
}
