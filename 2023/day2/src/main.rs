use regex;
use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    let answer1 = part1(input);
    let answer2 = part2(input);
    println!("{}", answer1);
    println!("{}", answer2);
}

fn match_color(color: &str, round: &str) -> i32 {
    let captures = Regex::new(format!(r"(\d+) {}", color).as_str())
        .unwrap()
        .captures(round);
    match captures {
        Some(cap) => cap.get(1).unwrap().as_str().parse::<i32>().unwrap(),
        None => 0,
    }
}

fn part1(input: &str) -> i32 {
    let green_cubes = 13;
    let red_cubes = 12;
    let blue_cubes = 14;
    let games = input.split("\n");

    let mut sum = 0;
    for game in games {
        let rounds = game.split(";");
        let mut impossible = false;
        for round in rounds {
            let red = match_color("red", round);
            let blue = match_color("blue", round);
            let green = match_color("green", round);
            impossible = (red > red_cubes) || (blue > blue_cubes) || (green > green_cubes);
            if impossible {
                break;
            }
        }
        // matches
        if !impossible {
            if let Some(game_id) = Regex::new(r"(\d+)").unwrap().captures(game).unwrap().get(1) {
                sum += game_id.as_str().parse::<i32>().unwrap();
            }
        }
    }

    return sum;
}

fn part2(input: &str) -> i32 {
    let games = input.split("\n");

    let mut sum = 0;
    for game in games {
        let mut green_cubes = 0;
        let mut red_cubes = 0;
        let mut blue_cubes = 0;

        let rounds = game.split(";");
        for round in rounds {
            let red = match_color("red", round);
            let blue = match_color("blue", round);
            let green = match_color("green", round);

            if red > red_cubes {
                red_cubes = red;
            }
            if blue > blue_cubes {
                blue_cubes = blue;
            }
            if green > green_cubes {
                green_cubes = green;
            }
        }

        let power = green_cubes * red_cubes * blue_cubes;
        sum += power;
    }
    return sum;
}
