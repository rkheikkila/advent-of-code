use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    let input = include_str!("../input.txt");
    let answer1 = part1(input);
    println!("{}", answer1);
    let answer2 = part2(input);
    println!("{}", answer2);
}

fn part1(input: &str) -> usize {
    input
        .split("\n")
        .map(|line| line.split_once(":").unwrap())
        .map(|(_, numbers)| numbers.split_once("|").unwrap())
        .map(|(left, right)| {
            let left_numbers: Vec<&str> = left.split_whitespace().collect();
            let matches = right
                .split_whitespace()
                .filter(|x| left_numbers.contains(x))
                .count();
            if matches >= 1 {
                return usize::pow(2, (matches - 1) as u32);
            } else {
                return 0;
            }
        })
        .sum()
}

#[derive(Debug)]
struct Game<'a> {
    game_id: &'a str,
    left_numbers: Vec<&'a str>,
    right_numbers: Vec<&'a str>,
}

fn matches(game: &Game) -> usize {
    game.right_numbers
        .iter()
        .filter(|x| game.left_numbers.contains(x))
        .count()
}

fn part2(input: &str) -> usize {
    let games: Vec<Game> = input
        .split("\n")
        .map(|line| line.split_once(":").unwrap())
        .map(|(game, numbers)| (game, numbers.split_once("|").unwrap()))
        .map(|(game, (left, right))| {
            let left_numbers: Vec<&str> = left.split_whitespace().collect();
            let right_numbers: Vec<&str> = right.split_whitespace().collect();
            Game {
                game_id: game,
                left_numbers: left_numbers,
                right_numbers: right_numbers,
            }
        })
        .collect();

    let mut match_counts: HashMap<usize, usize> =
        HashMap::from_iter(games.iter().enumerate().map(|(i, g)| (i, 1)));

    for i in 0..games.len() {
        let matches = matches(&games[i]);
        //dbg!(matches);
        let start = i + 1;
        let end = i + matches + 1;
        for j in start..end {
            let updated_count = match_counts.get(&j).unwrap_or(&1) + match_counts.get(&i).unwrap();
            match_counts.insert(j, updated_count);
        }
    }

    return match_counts.values().sum();
}
