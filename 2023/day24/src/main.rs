use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    part1(input);
    part2(input);
}

const EPS: f64 = 1e-4;

#[derive(Clone, Debug)]
struct Particle {
    initial_pos: (f64, f64, f64),
    velocity: (f64, f64, f64),
}

fn parse_coords(input: &str) -> (f64, f64, f64) {
    let mut iter = input.split(",").map(|num| num.trim().parse().unwrap());
    return (
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    );
}

fn find_collision_xy(p1: &Particle, p2: &Particle, check: bool) -> Option<(f64, f64)> {
    // eq 1: y = ax + c
    // eq 2: y = bx + d
    let a = p1.velocity.1 / p1.velocity.0;
    let b = p2.velocity.1 / p2.velocity.0;

    let c = p1.initial_pos.1 - a * p1.initial_pos.0;
    let d = p2.initial_pos.1 - b * p2.initial_pos.0;

    let denom = a - b;

    if denom.abs() < EPS {
        return None;
    }

    let ix = (d - c) / denom;
    let iy = c + a * ix;

    if check {
        if f64::signum(ix - p1.initial_pos.0) == f64::signum(p1.velocity.0)
            && f64::signum(ix - p2.initial_pos.0) == f64::signum(p2.velocity.0)
            && f64::signum(iy - p1.initial_pos.1) == f64::signum(p1.velocity.1)
            && f64::signum(iy - p2.initial_pos.1) == f64::signum(p2.velocity.1)
        {
            return Some((ix, iy));
        } else {
            return None;
        }
    } else {
        return Some((ix, iy));
    }
}

fn part1(input: &str) {
    let particles: Vec<Particle> = input
        .split("\n")
        .map(|line| {
            let (p, v) = line.split_once("@").unwrap();
            let pos = parse_coords(p);
            let velocity = parse_coords(v);
            Particle {
                initial_pos: pos,
                velocity: velocity,
            }
        })
        .collect();

    let low_bound = 200000000000000.0;
    let high_bound = 400000000000000.0;

    let points: Vec<Option<(f64, f64)>> = particles
        .iter()
        .tuple_combinations()
        .map(|(p1, p2)| find_collision_xy(p1, p2, true))
        .filter(|p| {
            p.map_or(false, |v| {
                v.0 >= low_bound && v.0 <= high_bound && v.1 >= low_bound && v.1 <= high_bound
            })
        })
        .collect();
    println!("{}", points.len());
}

fn part2(input: &str) {
    let particles: Vec<Particle> = input
        .split("\n")
        .map(|line| {
            let (p, v) = line.split_once("@").unwrap();
            let pos = parse_coords(p);
            let velocity = parse_coords(v);
            Particle {
                initial_pos: pos,
                velocity: velocity,
            }
        })
        .collect();

    let range = 1000;

    let mut potential_vx: HashSet<isize> = HashSet::new();
    let mut potential_vy: HashSet<isize> = HashSet::new();
    let mut potential_vz: HashSet<isize> = HashSet::new();

    for (p1, p2) in particles.iter().tuple_combinations() {
        if p1.velocity.0 == p2.velocity.0 {
            let mut vx_set: HashSet<isize> = HashSet::new();
            let diff = (p2.initial_pos.0 - p1.initial_pos.0) as isize;
            for v in -range..range {
                if v == p1.velocity.0 as isize {
                    continue;
                }
                if diff.rem_euclid(v - p1.velocity.0 as isize) == 0 {
                    vx_set.insert(v);
                }
            }
            if potential_vx.is_empty() {
                potential_vx.extend(vx_set.iter());
            } else {
                potential_vx = potential_vx.intersection(&vx_set).cloned().collect();
            }
        }
        if p1.velocity.1 == p2.velocity.1 {
            let mut vy_set: HashSet<isize> = HashSet::new();
            let diff = (p2.initial_pos.1 - p1.initial_pos.1) as isize;
            for v in -range..range {
                if v == p1.velocity.1 as isize {
                    continue;
                }
                if diff.rem_euclid(v - p1.velocity.1 as isize) == 0 {
                    vy_set.insert(v);
                }
            }
            if potential_vy.is_empty() {
                potential_vy.extend(vy_set.iter());
            } else {
                potential_vy = potential_vy.intersection(&vy_set).cloned().collect();
            }
        }
        if p1.velocity.2 == p2.velocity.2 {
            let mut vz_set: HashSet<isize> = HashSet::new();
            let diff = (p2.initial_pos.2 - p1.initial_pos.2) as isize;
            for v in -range..range {
                if v == p1.velocity.2 as isize {
                    continue;
                }
                if diff.rem_euclid(v - p1.velocity.2 as isize) == 0 {
                    vz_set.insert(v);
                }
            }
            if potential_vz.is_empty() {
                potential_vz.extend(vz_set.iter());
            } else {
                potential_vz = potential_vz.intersection(&vz_set).cloned().collect();
            }
        }
    }
    let vx = potential_vx.iter().nth(0).unwrap();
    let vy = potential_vy.iter().nth(0).unwrap();
    let vz = potential_vz.iter().nth(0).unwrap();

    let p1 = &particles[0];
    let p2 = &particles[1];

    let ma = (p1.velocity.1 - *vy as f64) / (p1.velocity.0 - *vx as f64);
    let mb = (p2.velocity.1 - *vy as f64) / (p2.velocity.0 - *vx as f64);

    let ca = p1.initial_pos.1 - ma * p1.initial_pos.0;
    let cb = p2.initial_pos.1 - mb * p2.initial_pos.0;
    let px = ((cb - ca) / (ma - mb)) as isize;
    let py = (ma * px as f64 + ca) as isize;
    let t = (px - p1.initial_pos.0 as isize) / (p1.velocity.0 as isize - vx);
    let pz = p1.initial_pos.2 as isize + (p1.velocity.2 as isize - vz) * t;
    dbg!(t);
    dbg!(px, py, pz);
    println!("{}", px + py + pz);
}
