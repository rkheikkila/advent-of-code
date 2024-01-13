use std::cmp::{max, min, Ord};
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use itertools::Itertools;
fn main() {
    let input = include_str!("../input.txt");
    day22(input);
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Brick {
    id: usize,
    start: (isize, isize, isize),
    end: (isize, isize, isize),
}

impl Brick {
    fn intersects(&self, other: &Brick) -> bool {
        let s1 = self.start;
        let s2 = other.start;
        let e1 = self.end;
        let e2 = other.end;

        let x_intersect = max(s1.0, s2.0) <= min(e1.0, e2.0);
        let y_intersect = max(s1.1, s2.1) <= min(e1.1, e2.1);

        return x_intersect && y_intersect;
    }
}

fn parse_coords(input: &str) -> (isize, isize, isize) {
    let mut iter = input.split(",").map(|num| num.parse().unwrap());
    return (
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    );
}

fn count_dependent_bricks(start: usize, supported_by: &HashMap<usize, Vec<usize>>) -> usize {
    let mut count = 0;
    let mut queue = VecDeque::from_iter([start]);
    let mut removed: HashSet<usize> = HashSet::from_iter([start]);
    while let Some(brick) = queue.pop_front() {
        let dependent_bricks: Vec<usize> = supported_by
            .iter()
            .filter_map(|(br, supports)| {
                if supports.iter().all(|x| removed.contains(x)) && !removed.contains(br) {
                    Some(*br)
                } else {
                    None
                }
            })
            .collect();

        for br in dependent_bricks {
            removed.insert(br);
            count += 1;
            queue.push_back(br);
        }
    }
    return count;
}

fn day22(input: &str) {
    let mut bricks: Vec<Brick> = input
        .split("\n")
        .enumerate()
        .map(|(i, line)| {
            let (start, end) = line.split_once("~").unwrap();
            let start_coords = parse_coords(start);
            let end_coords = parse_coords(end);
            Brick {
                id: i,
                start: start_coords,
                end: end_coords,
            }
        })
        .sorted_by(|a, b| Ord::cmp(&a.start.2, &b.start.2))
        .collect();

    let nbricks = bricks.len();

    let mut supports: HashMap<usize, Vec<usize>> = HashMap::new();

    for i in 0..nbricks {
        while bricks[i].start.2 > 1 {
            let mut falling = true;
            for j in 0..nbricks {
                if i != j && bricks[j].end.2 == bricks[i].start.2 - 1 {
                    if (&bricks[i]).intersects(&bricks[j]) {
                        supports
                            .entry(j)
                            .and_modify(|x| x.push(i))
                            .or_insert(vec![i]);
                        falling = false;
                    }
                }
            }
            if falling {
                bricks[i].start.2 -= 1;
                bricks[i].end.2 -= 1;
            } else {
                break;
            }
        }
    }

    for i in 0..nbricks {
        supports.entry(i).or_default();
    }

    // invert map
    let mut supported_by: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..nbricks {
        let blocks = supports.get(&i).unwrap();
        for block in blocks {
            supported_by.entry(*block).or_default().push(i);
        }
    }

    let mut removable: HashSet<usize> = HashSet::from_iter(0..nbricks);

    for (_, support) in supported_by.iter() {
        if support.len() == 1 {
            removable.remove(&support[0]);
        }
    }
    // part1
    println!("{:?}", removable.len());

    // part2
    let count: usize = (0..nbricks)
        .map(|b| count_dependent_bricks(b, &supported_by))
        .sum();
    println!("{}", count);
}

#[test]
fn test_intersect1() {
    let b1 = Brick {
        id: 0,
        start: (0, 0, 1),
        end: (0, 0, 1),
    };
    let b2 = b1.clone();
    assert!(b1.intersects(&b2));
}

#[test]
fn test_intersect2() {
    let b1 = Brick {
        id: 0,
        start: (0, 0, 1),
        end: (0, 3, 1),
    };
    let b2 = b1.clone();
    assert!(b1.intersects(&b2));
}

#[test]
fn test_intersect3() {
    let b1 = Brick {
        id: 0,
        start: (0, 0, 1),
        end: (0, 3, 1),
    };
    let b2 = Brick {
        id: 0,
        start: (0, 1, 1),
        end: (0, 1, 1),
    };
    assert!(b1.intersects(&b2));
}

#[test]
fn test_intersect4() {
    let b1 = Brick {
        id: 0,
        start: (0, 0, 1),
        end: (0, 3, 1),
    };
    let b2 = Brick {
        id: 0,
        start: (0, 3, 1),
        end: (0, 4, 1),
    };
    assert!(b1.intersects(&b2));
}

#[test]
fn test_intersect5() {
    let b1 = Brick {
        id: 0,
        start: (1, 1, 1),
        end: (2, 1, 1),
    };
    let b2 = Brick {
        id: 0,
        start: (2, 1, 1),
        end: (2, 3, 1),
    };
    assert!(b1.intersects(&b2));
}

#[test]
fn test_intersect6() {
    let b1 = Brick {
        id: 0,
        start: (1, 1, 1),
        end: (2, 1, 1),
    };
    let b2 = Brick {
        id: 0,
        start: (4, 1, 1),
        end: (5, 1, 1),
    };
    assert!(!b1.intersects(&b2));
}
