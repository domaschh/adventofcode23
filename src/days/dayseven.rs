use std::{collections::HashMap, time::Instant};

use super::utils::read_file;

const CHARS: [&'static str; 14] = [
    "A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2", "1",
];

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

#[derive(Debug)]
struct Hand {
    bid: u16,
    cards: (u8, u8, u8, u8, u8),
    ty: HandType,
}

impl Hand {
    fn new(cards: &str, bids: &str, part2: bool) -> Self {
        let v: Vec<u8> = cards
            .chars()
            .map(|char| {
                if part2 && char == 'J' {
                    0
                } else {
                    CHARS
                        .iter()
                        .rev()
                        .position(|x| x == &char.to_string())
                        .unwrap() as u8
                }
            })
            .collect();

        Self {
            bid: bids.parse::<u16>().unwrap(),
            cards: (v[0], v[1], v[2], v[3], v[4]),
            ty: Self::calc_strength(cards, part2),
        }
    }

    fn calc_strength(cards: &str, part2: bool) -> HandType {
        let mut dist: HashMap<char, u16> = HashMap::new();
        for char in cards.chars() {
            dist.entry(char).and_modify(|x| *x += 1).or_insert(1);
        }

        let mut scores: Vec<u16> = dist
            .keys()
            .map(|key| {
                if part2 && key == &'J' {
                    0_u16
                } else {
                    dist.get(key).unwrap().to_owned()
                }
            })
            .collect();
        scores.sort_by(|a, b| b.cmp(a));

        if part2 {
            scores[0] += cards.chars().filter(|a| a == &'J').count() as u16;
        }

        let strength_type = match scores[0] {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                if scores[1] == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            2 => {
                if scores[1] == 2 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            _ => HandType::HighCard,
        };

        strength_type
    }
}
pub(crate) fn dayseven1(filepath: &str) -> usize {
    let input = read_file(filepath).unwrap();
    let mut hands: Vec<Hand> = input
        .into_iter()
        .map(|line| {
            let s = line.split_once(" ").unwrap();
            Hand::new(s.0, s.1, false)
        })
        .collect();

    hands.sort_unstable_by_key(|hand| (hand.ty, hand.cards));

    let winnings = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (hand.bid as usize * (i + 1)));

    winnings
}

pub(crate) fn dayseven2(filepath: &str) -> usize {
    let input = read_file(filepath).unwrap();

    let mut hands: Vec<Hand> = input
        .into_iter()
        .map(|line| {
            let s = line.split_once(" ").unwrap();
            Hand::new(s.0, s.1, true)
        })
        .collect();

    hands.sort_unstable_by_key(|hand| (hand.ty, hand.cards));

    let winnings = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (hand.bid as usize * (i + 1)));

    winnings
}
