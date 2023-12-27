use std::cmp::Ordering;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Combination {
    cards: String,
    bid: usize,
    counts: HashMap<u8, u8>,
    ranks: HashMap<char, u8>
}

impl Eq for Combination {}

impl PartialOrd<Self> for Combination {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Combination {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut diff: u8 = 0;
        let mut max: u8 = 0;

        let mut i: u8 = 5;
        while i > 0 {
            let count = self.counts.get(&i).unwrap();
            let count_other = other.counts.get(&i).unwrap();

            if *count > 0 && max == 0 { max = i }

            if count != count_other {
                diff = i;
                break;
            }

            i -= 1;
        }

        if diff > 0 {
            return self.counts.get(&diff).unwrap()
                .cmp(other.counts.get(&diff).unwrap())
        }

        let cards = self.cards.chars();
        let cards_other = other.cards.chars();

        let mut diff = 0;
        for (i, card) in cards.clone().enumerate() {
            if card != cards_other.clone().into_iter().collect::<Vec<_>>()[i] {
                diff = i + 1;
                break;
            }
        }

        if diff > 0 {
            let card = cards.into_iter().collect::<Vec<_>>()[diff - 1];
            let card_other = cards_other.into_iter().collect::<Vec<_>>()[diff - 1];
            return CARDS.iter().rev().position(|c| c == &card).unwrap()
                .cmp(&CARDS.iter().rev().position(|c| c == &card_other).unwrap())
        }

        1.cmp(&1)
    }
}

impl PartialEq<Self> for Combination {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

static CARDS: [char; 13] = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];

impl From<&str> for Combination {
    fn from(value: &str) -> Self {
        let [cards, bid] = value
            .split_whitespace()
            .take(2)
            .collect::<Vec<_>>()[..]
            else { panic!("wrong data in file") };

        let cards = cards.to_string();
        let bid = bid.parse::<usize>().unwrap_or(0);
        let counts = counts(&cards);
        let ranks = ranks(&cards);


        Combination {
            cards,
            bid,
            counts,
            ranks,
        }
    }
}

impl Combination {
}

fn counts(card: &String) -> HashMap<u8, u8> {
    let h = ranks(card);
    let mut ranks: HashMap<u8, u8> = HashMap::new();

    for i in 1..6 {
        let count = h.values().filter(|v| v == &&i).count() as u8;
        ranks.insert(i, count);
    }

    ranks
}

fn ranks(card: &String) -> HashMap<char, u8> {
    let chars = card.chars();
    let mut h: HashMap<char, u8> = HashMap::new();

    for c in chars {
        match h.entry(c) {
            Occupied(mut entry) => {
                *entry.get_mut() += 1;
            }
            Vacant(entry) => {
                entry.insert(1);
            }
        }
    }

    h
}

fn main() {
    let now = std::time::Instant::now();
    let test = std::fs::read_to_string("input.txt").unwrap();
    let mut result: usize = 0;

    let mut combinations: Vec<Combination> = test.lines().map(|line| line.into()).collect();
    combinations.sort();

    for (i, c) in combinations.iter().enumerate() {
        result += (i + 1) * c.bid;
    }

    println!("result: {:?}, ({:?})", result, now.elapsed());
}

#[cfg(test)]
mod tests {
    use crate::Combination;

    #[test]
    fn part_1() {
        let test = std::fs::read_to_string("test.txt").unwrap();
        let mut result: usize = 0;

        let mut combinations: Vec<Combination> = test.lines().map(|line| line.into()).collect();
        combinations.sort();

        for (i, c) in combinations.iter().enumerate() {
            result += (i + 1) * c.bid;
        }

        assert_eq!(result, 6440)
    }

    #[test]
    fn part_2() {
        let test = std::fs::read_to_string("test.txt").unwrap();
        let result = 0;

        assert_eq!(result, 5905)
    }
}