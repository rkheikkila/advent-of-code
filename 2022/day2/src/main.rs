use std::io::BufRead;
use std::{fs::File, io};

#[derive(Clone, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

const move_order: [Move; 3] = [Move::Rock, Move::Paper, Move::Scissors];

fn find_index(m: &Move) -> usize {
    move_order.iter().position(|x| x == m).unwrap()
}

fn round_score(my_move: &Move, opponent_move: &Move) -> i32 {
    let my_index = find_index(my_move);
    let opponent_index = find_index(opponent_move);

    if my_index == opponent_index {
        return 3;
    } else {
        let win = my_index == ((opponent_index + 1) % 3);
        if win {
            return 6;
        } else {
            return 0;
        }
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let lines_iter = io::BufReader::new(file).lines();
    let lines = Vec::from_iter(lines_iter);

    let mut total_score = 0;

    for line in lines.iter() {
        if let Ok(value) = line {
            let mut split_value = value.split_whitespace();
            let opponent_move = match split_value.next().unwrap() {
                "A" => Move::Rock,
                "B" => Move::Paper,
                "C" => Move::Scissors,
                _ => panic!("unknown move"),
            };
            let my_move = match split_value.next().unwrap() {
                "X" => Move::Rock,
                "Y" => Move::Paper,
                "Z" => Move::Scissors,
                _ => panic!("unknown move"),
            };

            let mut score = 0;
            score += round_score(&my_move, &opponent_move);
            let move_score = match my_move {
                Move::Rock => 1,
                Move::Paper => 2,
                Move::Scissors => 3,
            };
            score += move_score;

            total_score += score;
        }
    }

    println!("{:}", total_score);

    let mut total_score_part2 = 0;
    // Score calculation for part 2
    for line in lines.iter() {
        if let Ok(value) = line {
            let mut split_value = value.split_whitespace();
            let opponent_move = match split_value.next().unwrap() {
                "A" => Move::Rock,
                "B" => Move::Paper,
                "C" => Move::Scissors,
                _ => panic!("unknown move"),
            };
            let my_move = match split_value.next().unwrap() {
                "X" => {
                    let move_index = find_index(&opponent_move);
                    let index: usize = ((3 + i32::try_from(move_index).unwrap() - 1) % 3)
                        .try_into()
                        .unwrap();
                    move_order[index].clone()
                }
                "Y" => opponent_move.clone(),
                "Z" => {
                    let move_index = find_index(&opponent_move);
                    move_order[(move_index + 1) % 3].clone()
                }
                _ => panic!("unknown move"),
            };

            let mut score = 0;
            score += round_score(&my_move, &opponent_move);
            let move_score = match my_move {
                Move::Rock => 1,
                Move::Paper => 2,
                Move::Scissors => 3,
            };
            score += move_score;

            total_score_part2 += score;
        }
    }

    println!("{:}", total_score_part2);
}
