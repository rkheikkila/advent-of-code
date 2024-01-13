fn main() {
    let input = include_str!("../input.txt");
    let a1 = part1(input);
    println!("{}", a1);
    let a2 = part2(input);
    println!("{}", a2);
}

fn extrapolate(v: &Vec<i32>, forward: bool) -> i32 {
    let diff: Vec<i32> = v.windows(2).map(|w| w[1] - w[0]).collect();
    let value = if forward { v.last() } else { v.first() };
    if diff.iter().all(|&x| x == 0) {
        return *value.unwrap();
    } else {
        let mult = if forward { 1 } else { -1 };
        let predicted = value.unwrap() + mult * extrapolate(&diff, forward);
        return predicted;
    }
}

fn part1(input: &str) -> i32 {
    input
        .split("\n")
        .map(|line| -> Vec<i32> {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .map(|v| extrapolate(&v, true))
        .sum()
}

fn part2(input: &str) -> i32 {
    input
        .split("\n")
        .map(|line| -> Vec<i32> {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .map(|v| extrapolate(&v, false))
        .sum()
}
