use std::cmp::Ordering;

use crate::util::fs_util::read_lines;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, PartialEq, PartialOrd, Eq)]
struct Hand {
    cards: Vec<u8>,
    bid: i32,
    rank: Option<i32>,
}

impl Hand {
    fn get_type(&self) -> HandType {
        /**
        Five of a kind, where all five cards have the same label: AAAAA
        Four of a kind, where four cards have the same label and one card has a different label: AA8AA
        Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
        Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
        Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
        One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
        High card, where all cards' labels are distinct: 23456
        */
        let mut max_match_count = 0;
        let mut max_match_symbol = 0;

        for card in &self.cards {
            let current_match_count = self
                .cards
                .iter()
                .filter(|v| v.clone() == card)
                .collect::<Vec<&u8>>()
                .len();

            if current_match_count > max_match_count {
                max_match_count = current_match_count;
                max_match_symbol = card.clone();
            }
        }

        if max_match_count == 5 {
            return HandType::FiveOfAKind;
        }

        if max_match_count == 4 {
            return HandType::FourOfAKind;
        }

        if max_match_count == 3 {
            let mut other_cards = self
                .cards
                .iter()
                .filter(|v| v != &&max_match_symbol)
                .collect::<Vec<&u8>>();

            // check if other cards are same
            if other_cards[0] == other_cards[1] {
                return HandType::FullHouse;
            } else {
                return HandType::ThreeOfAKind;
            }
        }

        if max_match_count == 2 {
            let mut other_cards = self
                .cards
                .iter()
                .filter(|v| v != &&max_match_symbol)
                .collect::<Vec<&u8>>();

            let mut max_match_count = 0;

            for card in &other_cards {
                let current_match_count = other_cards
                    .iter()
                    .filter(|v| v == &card)
                    .collect::<Vec<&&u8>>()
                    .len();

                if current_match_count > max_match_count {
                    max_match_count = current_match_count;
                }
            }

            if max_match_count == 2 {
                return HandType::TwoPair;
            } else {
                return HandType::OnePair;
            }
        }

        return HandType::HighCard;
    }

    fn get_type_2(&self) -> HandType {
        /**
        Five of a kind, where all five cards have the same label: AAAAA
        Four of a kind, where four cards have the same label and one card has a different label: AA8AA
        Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
        Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
        Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
        One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
        High card, where all cards' labels are distinct: 23456
         */
        let mut max_match_count = 0;
        let mut max_match_symbol = 0;

        for card in &self.cards {
            let current_match_count = self
                .cards
                .iter()
                .filter(|v| v.clone() == card || **v == 1 as u8)
                .collect::<Vec<&u8>>()
                .len();

            if current_match_count > max_match_count {
                max_match_count = current_match_count;
                max_match_symbol = card.clone();
            }
        }

        if max_match_count == 5 {
            return HandType::FiveOfAKind;
        }

        if max_match_count == 4 {
            return HandType::FourOfAKind;
        }

        if max_match_count == 3 {
            let mut other_cards = self
                .cards
                .iter()
                .filter(|v| v != &&max_match_symbol && **v != 1)
                .collect::<Vec<&u8>>();

            // check if other cards are same
            if other_cards[0] == other_cards[1] {
                return HandType::FullHouse;
            } else {
                return HandType::ThreeOfAKind;
            }
        }

        if max_match_count == 2 {
            let mut other_cards = self
                .cards
                .iter()
                .filter(|v| v != &&max_match_symbol && **v != 1)
                .collect::<Vec<&u8>>();

            let mut max_match_count = 0;

            for card in &other_cards {
                let current_match_count = other_cards
                    .iter()
                    .filter(|v| v == &card)
                    .collect::<Vec<&&u8>>()
                    .len();

                if current_match_count > max_match_count {
                    max_match_count = current_match_count;
                }
            }

            if max_match_count == 2 {
                return HandType::TwoPair;
            } else {
                return HandType::OnePair;
            }
        }

        return HandType::HighCard;
    }

    fn set_rank(&mut self, rank: i32) {
        self.rank = Some(rank);
    }

    fn get_power(&self) -> i32 {
        self.bid * self.rank.unwrap()
    }
}

pub fn day_7_camel_cards() -> i32 {
    let lines = read_lines("./inputs/day_7/values.txt").unwrap();

    let mut hands: Vec<Hand> = Vec::new();

    for line in lines {
        let line = line.unwrap();

        let (cards, bid) = line.split_once(" ").unwrap();

        let cards = cards
            .trim()
            .to_uppercase()
            .split("")
            .filter(|v| !v.is_empty())
            .map(|v| {
                if v.eq("A") {
                    14
                } else if v.eq("K") {
                    13
                } else if v.eq("Q") {
                    12
                } else if v.eq("J") {
                    11
                } else if v.eq("T") {
                    10
                } else {
                    v.parse().unwrap()
                }
            })
            .collect::<Vec<u8>>();

        hands.push(Hand {
            cards,
            bid: bid.parse().unwrap(),
            rank: None,
        });
    }

    hands.sort_by(|current, next| {
        let self_type = current.get_type();
        let other_type = next.get_type();

        if self_type > other_type {
            return Ordering::Less;
        } else if self_type < other_type {
            return Ordering::Greater;
        }
        // compare card order
        for i in 0..current.cards.len() {
            if current.cards[i] > next.cards[i] {
                return Ordering::Greater;
            } else if current.cards[i] < next.cards[i] {
                return Ordering::Less;
            }
        }

        Ordering::Equal
    });

    let mut total_power = 0;
    for (index, mut hand) in hands.iter_mut().enumerate() {
        hand.set_rank((index + 1) as i32);
        total_power = total_power + hand.get_power();
    }

    total_power
}

pub fn day_7_camel_cards_part2() -> i32 {
    let lines = read_lines("./inputs/day_7/values.txt").unwrap();

    let mut hands: Vec<Hand> = Vec::new();

    for line in lines {
        let line = line.unwrap();

        let (cards, bid) = line.split_once(" ").unwrap();

        let cards = cards
            .trim()
            .to_uppercase()
            .split("")
            .filter(|v| !v.is_empty())
            .map(|v| {
                if v.eq("A") {
                    14
                } else if v.eq("K") {
                    13
                } else if v.eq("Q") {
                    12
                } else if v.eq("J") {
                    1
                } else if v.eq("T") {
                    10
                } else {
                    v.parse().unwrap()
                }
            })
            .collect::<Vec<u8>>();

        hands.push(Hand {
            cards,
            bid: bid.parse().unwrap(),
            rank: None,
        });
    }

    hands.sort_by(|current, next| {
        let self_type = current.get_type_2();
        let other_type = next.get_type_2();

        if self_type > other_type {
            return Ordering::Less;
        } else if self_type < other_type {
            return Ordering::Greater;
        }
        // compare card order
        for i in 0..current.cards.len() {
            if current.cards[i] > next.cards[i] {
                return Ordering::Greater;
            } else if current.cards[i] < next.cards[i] {
                return Ordering::Less;
            }
        }

        Ordering::Equal
    });

    let mut total_power = 0;
    for (index, mut hand) in hands.iter_mut().enumerate() {
        hand.set_rank((index + 1) as i32);
        total_power = total_power + hand.get_power();
    }

    total_power
}
