use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    let input = include_str!("../input.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let lines: Vec<(&str, u32)> = input
        .split("\n")
        .map(|line| {
            let (head, _) = line.split_once(" (").unwrap();
            let (direction, distance) = head.split_once(" ").unwrap();
            (direction, distance.parse::<u32>().unwrap())
        })
        .collect();

    let mut points: Vec<(isize, isize)> = Vec::from_iter([(0, 0)]);
    let mut last_point = (0, 0);
    for (direction, distance) in lines {
        let delta = match direction {
            "D" => (0, -1),
            "U" => (0, 1),
            "R" => (1, 0),
            "L" => (-1, 0),
            _ => panic!("Unknown direction"),
        };

        for d in 1..=distance {
            let new_point = (
                last_point.0 + delta.0 as isize * d as isize,
                last_point.1 + delta.1 as isize * d as isize,
            );
            points.push(new_point);
        }
        last_point = *points.last().unwrap();
    }

    assert!(points.first().unwrap() == points.last().unwrap());

    let point_set: HashSet<(isize, isize)> = HashSet::from_iter(points);

    // BFS to count points in the interior
    let mut visited = HashSet::new();
    let mut queue: VecDeque<(isize, isize)> = VecDeque::new();
    let start_point = (1, -1);
    queue.push_back(start_point);

    let deltas = vec![(-1, 0), (0, -1), (1, 0), (0, 1)];

    while let Some(point) = queue.pop_front() {
        if visited.contains(&point) {
            continue;
        }
        visited.insert(point);
        for (dx, dy) in deltas.iter() {
            let new_x = point.0 + *dx as isize;
            let new_y = point.1 + *dy as isize;
            let interior_point = !point_set.contains(&(new_x, new_y));
            if interior_point && !visited.contains(&(new_x, new_y)) {
                queue.push_back((new_x, new_y));
            }
        }
        if visited.len().rem_euclid(10000) == 0 {
            println!("Visited {} points", visited.len());
        }
    }

    let area = visited.len() + point_set.len();
    println!("{}", area);
}

fn part2(input: &str) {
    let lines: Vec<(&str, u32)> = input
        .split("\n")
        .map(|line| {
            let (_, tail) = line.split_once(" (#").unwrap();
            let distance = u32::from_str_radix(&tail[0..5], 16).unwrap(); // drop right parenthesis
            let direction = match &tail[5..6] {
                "0" => "R",
                "1" => "D",
                "2" => "L",
                "3" => "U",
                _ => panic!("unknown direction"),
            };
            (direction, distance)
        })
        .collect();

    let mut line_length = 0;
    let mut points: Vec<(isize, isize)> = Vec::from_iter([(0, 0)]);
    let mut last_point = (0, 0);
    for (direction, distance) in lines {
        let delta = match direction {
            "D" => (0, -1),
            "U" => (0, 1),
            "R" => (1, 0),
            "L" => (-1, 0),
            _ => panic!("Unknown direction"),
        };

        let new_point = (
            last_point.0 + delta.0 as isize * distance as isize,
            last_point.1 + delta.1 as isize * distance as isize,
        );
        line_length += distance;
        points.push(new_point);
        last_point = new_point;
    }

    // Shoelace formula for polygon's area

    let mut area = 0;
    for i in 0..&points.len() - 1 {
        let (x1, y1) = points[i];
        let (x2, y2) = points[i + 1];

        area += (x1 * y2 - y1 * x2);
    }
    println!("{}", area.abs() / 2);

    // Get number of interior points from Pick's theorem
    let total_area = (area.abs() / 2) as usize + (line_length as usize / 2) + 1;
    println!("{}", total_area);
}
