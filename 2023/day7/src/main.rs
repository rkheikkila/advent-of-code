use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let answer = part2(input);
    println!("{}", answer);
}

#[derive(Debug, Ord, Eq, PartialOrd, PartialEq, Clone)]
enum Card {
    A,
    K,
    Q,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    J,
}

#[derive(Debug, Ord, Eq, PartialOrd, PartialEq, Clone)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPairs,
    Pair,
    HighCard,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Hand {
    hand_type: HandType,
    cards: Vec<Card>,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let order = self.hand_type.cmp(&other.hand_type);
        match order {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            Ordering::Equal => {
                for i in 0..5 {
                    let self_card = &self.cards[i];
                    let other_card = &other.cards[i];
                    let ordering = self_card.cmp(other_card);
                    if ordering != Ordering::Equal {
                        return ordering;
                    }
                }
                panic!("help");
            }
        }
    }
}

fn match_card(c: &char) -> Card {
    let card = match c {
        'A' => Card::A,
        'K' => Card::K,
        'Q' => Card::Q,
        'J' => Card::J,
        'T' => Card::T,
        '9' => Card::Nine,
        '8' => Card::Eight,
        '7' => Card::Seven,
        '6' => Card::Six,
        '5' => Card::Five,
        '4' => Card::Four,
        '3' => Card::Three,
        '2' => Card::Two,
        _ => panic!("unknown char"),
    };
    return card;
}

fn parse_hand(input: &str) -> Hand {
    let cards = input.chars().map(|c| match_card(&c)).collect();
    let mut char_counts: HashMap<char, u32> = HashMap::new();
    for c in input.chars() {
        let count = char_counts.get(&c).unwrap_or(&0) + 1;
        char_counts.insert(c, count);
    }

    let mut card_counts: Vec<(&char, &u32)> = char_counts.iter().collect();

    card_counts.sort_by(|(c1, k1), (c2, k2)| k2.cmp(k1));

    let j_count = char_counts.get(&'J').unwrap_or(&0);
    let (_, first_count) = card_counts
        .iter()
        .find(|(c, _)| **c != 'J')
        .unwrap_or(&(&'J', &0));
    let first_count = first_count.clone() + j_count.clone();
    match first_count {
        5 => {
            return Hand {
                hand_type: HandType::FiveOfKind,
                cards: cards,
            }
        }
        4 => {
            return Hand {
                hand_type: HandType::FourOfKind,
                cards: cards,
            }
        }
        3 => {
            let (_, second_count) = card_counts[1];
            if *second_count == 2 {
                return Hand {
                    hand_type: HandType::FullHouse,
                    cards: cards,
                };
            } else {
                return Hand {
                    hand_type: HandType::ThreeOfKind,
                    cards: cards,
                };
            }
        }
        2 => {
            let (_, second_count) = card_counts[1];
            if *second_count == 2 {
                return Hand {
                    hand_type: HandType::TwoPairs,
                    cards: cards,
                };
            } else {
                return Hand {
                    hand_type: HandType::Pair,
                    cards: cards,
                };
            }
        }
        1 => {
            return Hand {
                hand_type: HandType::HighCard,
                cards: cards,
            };
        }
        _ => panic!("unknown count"),
    }
}

fn part2(input: &str) -> u32 {
    let mut hands_and_bids: Vec<(Hand, u32)> = input
        .split("\n")
        .map(|line| line.split_once(" ").unwrap())
        .map(|(hand, bid)| (parse_hand(hand), bid.parse::<u32>().unwrap()))
        .collect();

    let count = hands_and_bids.len();
    hands_and_bids.sort_by(|(h1, _), (h2, _)| h1.cmp(h2));

    let mut sum = 0;
    for i in 0..count {
        let rank = count - i;
        let bid = hands_and_bids[i].1;
        sum += rank as u32 * bid;
    }
    return sum;
}
