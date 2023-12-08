use itertools::Itertools;

use super::utils::read_file;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone)]
struct Hand {
    cards: [char; 5],
    ty: HandType,
    bid: i32,
}

pub(crate) fn dayseven1(filepath: &str) -> Result<i32, String> {
    let lines = read_file(filepath).map_err(|_| "Reading file failed")?;
    let res: Vec<Hand> = lines
        .iter()
        .filter_map(|line| {
            if let Some((card_str, bid_str)) = line.split_once(" ") {
                let mut cards: [char; 5] = ['0'; 5];
                card_str
                    .chars()
                    .enumerate()
                    .take(5)
                    .for_each(|(i, c)| cards[i] = c);

                Some(chars_to_hand(cards, bid_str.parse::<i32>().ok()?))
            } else {
                None
            }
        })
        .collect();
    Ok(1)
}

fn chars_to_hand(cards: [char; 5], bid: i32) -> Hand {
    let occurrences: HashMap<char, usize> = cards.iter().fold(HashMap::new(), |mut acc, &c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });
    let biggest_occurence = *occurrences.values().max().unwrap_or(&0);
    let distinct_count = occurrences.len();
    dbg!(biggest_occurence);
    dbg!(distinct_count);
    let ty = match (biggest_occurence, distinct_count) {
        (5, _) => HandType::FiveOfAKind,
        (4, _) => HandType::FourOfAKind,
        (3, 2) => HandType::FullHouse,
        (3, _) => HandType::ThreeOfAKind,
        (2, 3) => HandType::TwoPair,
        (2, _) => HandType::OnePair,
        (1, _) => HandType::HighCard,
        _ => HandType::HighCard,
    };
    Hand { cards, ty, bid }
}

pub(crate) fn dayseven2(filepath: &str) -> Result<i32, String> {
    Ok(1)
}

#[test]
fn test_highcard() {
    let cards: [char; 5] = ['A', 'B', 'C', 'D', 'E'];
    let bid = 0;
    let hand = chars_to_hand(cards, bid);
    assert_eq!(hand.ty, HandType::HighCard);
}

#[test]
fn test_pair() {
    let cards: [char; 5] = ['A', 'B', 'C', 'A', 'E'];
    let bid = 0;
    let hand = chars_to_hand(cards, bid);
    assert_eq!(hand.ty, HandType::OnePair);
}
#[test]
fn test_two_pair() {
    let cards: [char; 5] = ['A', 'B', 'C', 'A', 'B'];
    let bid = 0;
    let hand = chars_to_hand(cards, bid);
    assert_eq!(hand.ty, HandType::TwoPair);
}

#[test]
fn test_tripple() {
    let cards: [char; 5] = ['A', 'B', 'C', 'A', 'A'];
    let bid = 0;
    let hand = chars_to_hand(cards, bid);
    assert_eq!(hand.ty, HandType::ThreeOfAKind);
}

#[test]
fn test_fullhouse() {
    let cards: [char; 5] = ['A', 'B', 'B', 'A', 'A'];
    let bid = 0;
    let hand = chars_to_hand(cards, bid);
    assert_eq!(hand.ty, HandType::FullHouse);
}

#[test]
fn test_fourokind() {
    let cards: [char; 5] = ['A', 'B', 'A', 'A', 'A'];
    let bid = 0;
    let hand = chars_to_hand(cards, bid);
    assert_eq!(hand.ty, HandType::FourOfAKind);
}

#[test]
fn test_fiveokind() {
    let cards: [char; 5] = ['A', 'A', 'A', 'A', 'A'];
    let bid = 0;
    let hand = chars_to_hand(cards, bid);
    assert_eq!(hand.ty, HandType::FiveOfAKind);
}
