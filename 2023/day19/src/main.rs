use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    part1(input);
    part2(input);
}

#[derive(Clone, Copy, Debug)]
enum Operator {
    GreaterThan,
    LessThan,
    Any,
}

#[derive(Debug)]
struct DecisionNode<'a> {
    attribute: char,
    operator: Operator,
    value: u32,
    destination: &'a str,
}

impl<'a> DecisionNode<'a> {
    fn evaluate(&'a self, map: &HashMap<char, u32>) -> Option<&str> {
        let map_value = map.get(&self.attribute);
        match (self.operator, map_value) {
            (Operator::Any, _) => return Some(self.destination),
            (_, None) => return None,
            (Operator::LessThan, Some(value)) => {
                if value < &self.value {
                    return Some(self.destination);
                } else {
                    return None;
                }
            }
            (Operator::GreaterThan, Some(value)) => {
                if value > &self.value {
                    return Some(self.destination);
                } else {
                    return None;
                }
            }
        }
    }
}

fn parse_decision_node(input: &str) -> DecisionNode {
    let operator_regex = Regex::new(r"[<>]").unwrap();
    let matched_op = operator_regex.find(input);

    match matched_op {
        Some(m) => {
            let op = m.as_str();
            let lt = op == "<";
            let (attr, tail) = input.split_once(op).unwrap();
            let (value, destination) = tail.split_once(":").unwrap();
            let operator = match lt {
                true => Operator::LessThan,
                false => Operator::GreaterThan,
            };
            return DecisionNode {
                attribute: attr.chars().nth(0).unwrap(),
                operator: operator,
                value: value.parse::<u32>().unwrap(),
                destination: destination,
            };
        }
        None => {
            return DecisionNode {
                attribute: '0',
                operator: Operator::Any,
                value: 0,
                destination: input,
            };
        }
    }
}

fn evaluate(
    part: &HashMap<char, u32>,
    workflows: &HashMap<&str, Vec<DecisionNode>>,
    start: &str,
) -> bool {
    let nodes = workflows.get(start).unwrap();
    for node in nodes {
        let dest = node.evaluate(&part);
        match dest {
            Some("R") => return false,
            Some("A") => return true,
            Some(dest) => return evaluate(part, workflows, dest),
            None => continue,
        }
    }
    return false;
}

fn part1(input: &str) {
    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let workflows = workflows.split("\n").map(|line| {
        let (name, rules) = line.split_once("{").unwrap();
        let rules = rules.strip_suffix("}").unwrap();
        let nodes: Vec<DecisionNode> = rules
            .split(",")
            .map(|rule| parse_decision_node(rule))
            .collect();
        (name, nodes)
    });

    let parts: Vec<HashMap<char, u32>> = parts
        .split("\n")
        .map(|line| {
            let contents = line.strip_prefix("{").unwrap().strip_suffix("}").unwrap();
            let attributes: Vec<(char, u32)> = contents
                .split(",")
                .map(|item| {
                    let (attr, value) = item.split_once("=").unwrap();
                    //dbg!(attr, value);
                    (attr.chars().nth(0).unwrap(), value.parse::<u32>().unwrap())
                })
                .collect();
            let map = HashMap::from_iter(attributes);
            map
        })
        .collect();

    let workflow_map: HashMap<&str, Vec<DecisionNode<'_>>> = HashMap::from_iter(workflows);

    let mut sum = 0;
    for part in parts {
        let accept = evaluate(&part, &workflow_map, "in");
        if accept {
            sum += part.values().sum::<u32>();
        }
    }
    println!("{}", sum);
}

#[derive(Clone, Copy, Debug)]
struct Interval {
    low: u32,
    high: u32,
}

impl Interval {
    fn length(&self) -> u32 {
        return self.high - self.low + 1;
    }

    fn intersect(&self, other: &Interval) -> Interval {
        if other.low > self.high || self.low > other.high {
            // Trick to get interval of length 0
            return Interval { low: 1, high: 0 };
        } else {
            return Interval {
                low: max(self.low, other.low),
                high: min(self.high, other.high),
            };
        }
    }
}

fn evaluate_intervals(
    part: &HashMap<char, Interval>,
    workflows: &HashMap<&str, Vec<DecisionNode>>,
    start: &str,
) -> Vec<HashMap<char, Interval>> {
    if start == "A" {
        return Vec::from_iter([part.clone()]);
    } else if start == "R" {
        return Vec::new();
    }
    let nodes = workflows.get(start).unwrap();
    let mut intervals = Vec::new();
    let mut map = part.clone();

    // At each iteration, if operator is < or >, create two cases
    // 1. where the condition is true, which is solved recursively
    // 2. where condition false, where map is modified in place
    //    and passed on to the next decision node
    for node in nodes {
        match (node.operator, node.destination) {
            (Operator::Any, dest) => {
                intervals.extend(evaluate_intervals(&map.clone(), &workflows, dest))
            }
            (Operator::LessThan, dest) => {
                let interval = map.get(&node.attribute).unwrap();
                let lt_interval = Interval {
                    low: 1,
                    high: node.value - 1,
                };

                let mut left_map = map.clone();
                left_map.insert(node.attribute, lt_interval.intersect(interval));
                intervals.extend(evaluate_intervals(&left_map, &workflows, dest));

                // Modify map for next iteration
                let gt_interval = Interval {
                    low: node.value,
                    high: 100000,
                };
                map.insert(node.attribute, gt_interval.intersect(interval));
            }
            (Operator::GreaterThan, dest) => {
                let interval = map.get(&node.attribute).unwrap();
                let gt_interval = Interval {
                    low: node.value + 1,
                    high: 100000,
                };
                let mut right_map = map.clone();
                right_map.insert(node.attribute, gt_interval.intersect(interval));
                intervals.extend(evaluate_intervals(&right_map, &workflows, dest));

                // Modify map for next iteration
                let lt_interval = Interval {
                    low: 1,
                    high: node.value,
                };
                map.insert(node.attribute, lt_interval.intersect(interval));
            }
        }
    }
    return intervals;
}

fn part2(input: &str) {
    let (workflows, _) = input.split_once("\n\n").unwrap();
    let workflows = workflows.split("\n").map(|line| {
        let (name, rules) = line.split_once("{").unwrap();
        let rules = rules.strip_suffix("}").unwrap();
        let nodes: Vec<DecisionNode> = rules
            .split(",")
            .map(|rule| parse_decision_node(rule))
            .collect();
        (name, nodes)
    });

    let workflow_map: HashMap<&str, Vec<DecisionNode<'_>>> = HashMap::from_iter(workflows);

    let part: HashMap<char, Interval> = HashMap::from_iter([
        ('x', Interval { low: 1, high: 4000 }),
        ('m', Interval { low: 1, high: 4000 }),
        ('a', Interval { low: 1, high: 4000 }),
        ('s', Interval { low: 1, high: 4000 }),
    ]);

    let combinations: usize = evaluate_intervals(&part, &workflow_map, "in")
        .iter()
        .map(|x| x.values().map(|iv| iv.length() as usize).product::<usize>())
        .sum();
    println!("{}", combinations);
}
