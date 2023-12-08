use super::utils::read_file;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

#[derive(Debug, Clone)]
struct Hand {
    cards: [char; 5],
    ty: HandType,
    bid: i64,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.ty == other.ty && self.cards == other.cards {
            true
        } else {
            false
        }
    }
}
impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            Some(std::cmp::Ordering::Equal)
        } else {
            if self.ty != other.ty {
                Some(self.ty.cmp(&other.ty))
            } else {
                for (&myc, &othc) in self.cards.iter().zip(other.cards.iter()) {
                    if myc == othc {
                        continue;
                    } else {
                        if myc.is_digit(10) && othc.is_digit(10) {
                            return Some(myc.cmp(&othc));
                        } else if othc.is_digit(10) && !myc.is_digit(10) {
                            return Some(std::cmp::Ordering::Greater);
                        } else if myc.is_digit(10) && !othc.is_digit(10) {
                            return Some(std::cmp::Ordering::Less);
                        } else {
                            return Some(myc.cmp(&othc).reverse());
                        }
                    }
                }
                unreachable!()
            }
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == other {
            std::cmp::Ordering::Equal
        } else {
            if self.ty != other.ty {
                self.ty.cmp(&other.ty)
            } else {
                for (&myc, &othc) in self.cards.iter().zip(other.cards.iter()) {
                    if myc == othc {
                        continue;
                    } else {
                        if myc.is_digit(10) && othc.is_digit(10) {
                            return myc.cmp(&othc);
                        } else if othc.is_digit(10) && !myc.is_digit(10) {
                            return std::cmp::Ordering::Greater;
                        } else if myc.is_digit(10) && !othc.is_digit(10) {
                            return std::cmp::Ordering::Less;
                        } else {
                            return myc.cmp(&othc);
                        }
                    }
                }
                unreachable!()
            }
        }
    }
}
pub(crate) fn dayseven1(filepath: &str) -> Result<i64, String> {
    let lines = read_file(filepath).map_err(|_| "Reading file failed")?;
    let mut res: Vec<Hand> = lines
        .iter()
        .filter_map(|line| {
            let (card_str, bid_str) = line.split_once(" ")?;
            let mut cards: [char; 5] = ['0'; 5];
            card_str
                .chars()
                .enumerate()
                .take(5)
                .for_each(|(i, c)| cards[i] = c);

            Some(chars_to_hand(cards, bid_str.parse::<i64>().ok()?))
        })
        .collect();
    res.sort();
    Ok(res
        .iter()
        .enumerate()
        .map(|(i, hand)| {
            // println!("{:?}", hand);
            // println!(
            //     "{:?} + {:?} = {:?}",
            //     i + 1,
            //     hand.bid,
            //     (i + 1) as i64 * hand.bid
            // );
            (i + 1) as i64 * hand.bid
        })
        .sum())
}

fn chars_to_hand(cards: [char; 5], bid: i64) -> Hand {
    let occurrences: HashMap<char, usize> = cards.iter().fold(HashMap::new(), |mut acc, &c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });
    let biggest_occurence = *occurrences.values().max().unwrap_or(&0);
    let distinc_card_count = occurrences.len();
    let ty = match (biggest_occurence, distinc_card_count) {
        (5, _) => HandType::FiveOfAKind,
        (4, _) => HandType::FourOfAKind,
        (3, 2) => HandType::FullHouse,
        (3, _) => HandType::ThreeOfAKind,
        (2, 3) => HandType::TwoPair,
        (2, 4) => HandType::OnePair,
        _ => HandType::HighCard,
    };
    Hand { cards, ty, bid }
}

pub(crate) fn dayseven2(filepath: &str) -> Result<i64, String> {
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

#[test]
fn cards_are_eq() {
    let hand1 = Hand {
        ty: HandType::OnePair,
        cards: ['A'; 5],
        bid: 3,
    };
    let hand2 = Hand {
        ty: HandType::OnePair,
        cards: ['A'; 5],
        bid: 3,
    };

    assert_eq!(hand1, hand2)
}

#[test]
fn test_strength_3() {
    let hand1 = Hand {
        ty: HandType::FullHouse,
        cards: ['K', 'A', '8', '8', '8'],
        bid: 3,
    };
    let hand2 = Hand {
        ty: HandType::FullHouse,
        cards: ['T', 'B', '8', '8', '8'],
        bid: 3,
    };

    let mut v = vec![hand1, hand2];
    v.sort();
    assert_eq!(v.get(0).unwrap().cards[0], 'T');
    assert_eq!(v.get(1).unwrap().cards[0], 'K');
}

#[test]
fn test_strength_4() {
    let hand1 = Hand {
        ty: HandType::FullHouse,
        cards: ['3', '3', '8', '8', '8'],
        bid: 3,
    };
    let hand2 = Hand {
        ty: HandType::FullHouse,
        cards: ['2', '2', '8', '8', '8'],
        bid: 3,
    };

    let mut v = vec![hand1, hand2];
    v.sort();
    assert_eq!(v.get(0).unwrap().cards[0], '2');
    assert_eq!(v.get(1).unwrap().cards[0], '3');
}

#[test]
fn test_strength_5() {
    let hand1 = Hand {
        ty: HandType::FullHouse,
        cards: ['3', '3', '8', '8', '8'],
        bid: 3,
    };
    let hand2 = Hand {
        ty: HandType::FullHouse,
        cards: ['Q', 'Q', '8', '8', '8'],
        bid: 3,
    };

    let mut v = vec![hand1, hand2];
    v.sort();
    assert_eq!(v.get(0).unwrap().cards[0], '3');
    assert_eq!(v.get(1).unwrap().cards[0], 'Q');
}

#[test]
fn test_strength_6() {
    let hand1 = Hand {
        ty: HandType::FullHouse,
        cards: ['3', '3', '8', '8', '8'],
        bid: 3,
    };
    let hand2 = Hand {
        ty: HandType::FullHouse,
        cards: ['Q', 'Q', '8', '8', '8'],
        bid: 3,
    };

    let mut v = vec![hand2, hand1];
    v.sort();
    assert_eq!(v.get(0).unwrap().cards[0], '3');
    assert_eq!(v.get(1).unwrap().cards[0], 'Q');
}
