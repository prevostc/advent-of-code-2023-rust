use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;
use std::cmp::Ordering::{self, Equal, Greater, Less};
use std::fmt::Display;

advent_of_code::solution!(7);

#[derive(Debug, Copy, Clone, Eq, Hash, Ord)]
enum Card {
    Value(i32),
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        Card::Value(
            "23456789TJQKA"
                .chars()
                .position(|c| c == value)
                .unwrap()
                .try_into()
                .unwrap(),
        )
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Card::Value(v) => "23456789TJQKA"
                .chars()
                .enumerate()
                .find(|(i, _)| (*i as i32).eq(v)),
        }
        .map(|(_, c)| c)
        .unwrap();
        write!(f, "{}", str)
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Card::Value(a), Card::Value(b)) => a.eq(b),
        }
    }
}
impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Card::Value(a), Card::Value(b)) => Some(a.cmp(b)),
        }
    }
}

#[derive(Debug, Clone, Eq)]
struct Hand {
    cards: [Card; 5],
    counts: Vec<(Card, usize)>,
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let mut cards = [Card::Value(0); 5];
        value
            .chars()
            .map(Card::from)
            .enumerate()
            .for_each(|(i, c)| {
                cards[i] = c;
            });

        let counts: Vec<(Card, usize)> = cards
            .into_iter()
            .counts()
            .into_iter()
            .sorted_by(
                |(card_a, count_a), (card_b, count_b)| match count_b.cmp(count_a) {
                    Equal => card_b.cmp(card_a),
                    o => o,
                },
            )
            .map(|(c, v)| (c, v))
            .collect::<Vec<_>>();
        Hand { cards, counts }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards.iter().zip(other.cards).all(|(a, b)| a.eq(&b))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let len_cmp = other.counts.len().cmp(&self.counts.len());
        if Equal != len_cmp {
            return len_cmp;
        }
        let len_cmp = other
            .counts
            .iter()
            .filter(|(c, i)| *i > 1)
            .count()
            .cmp(&self.counts.iter().filter(|(c, i)| *i > 1).count());
        if Equal != len_cmp {
            return len_cmp;
        }

        for m in self.cards.iter().zip(&other.cards).map(|(a, b)| a.cmp(b)) {
            if m != Equal {
                return m;
            }
        }
        return Equal;
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.cards.iter().for_each(|c| write!(f, "{}", *c).unwrap());
        Ok(())
    }
}

#[derive(Debug, Clone, Eq)]
struct Game {
    hand: Hand,
    bets: u64,
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.hand.partial_cmp(&other.hand)
    }
}
impl Ord for Game {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand.cmp(&other.hand)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    // maybe we can cheat a bit
    let ordered_games = input
        .lines()
        .map(|l| {
            l.split_once(" ")
                .map(|(cards, b)| Game {
                    hand: Hand::from(cards),
                    bets: b.parse::<u64>().unwrap(),
                })
                .unwrap()
        })
        .sorted()
        .collect::<Vec<_>>();

    for g in &ordered_games {
        println!("{} {}", g.hand, g.bets);
    }

    let res = ordered_games
        .iter()
        .enumerate()
        .map(|(i, g)| (i as u64 + 1) * g.bets)
        .sum();
    //dbg!(ordered_games);
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_format() {
        dbg!(Card::from('K'));
        println!("{}", Card::from('K'));
    }
    #[test]
    fn test_hand_format() {
        println!("{}", Hand::from("KQTJ4"));
        println!("{}", Hand::from("23456"));
    }

    #[test]
    fn test_card_ord() {
        assert_eq!(Card::from('K').cmp(&Card::from('K')), Equal);
        assert_eq!(Card::from('K').cmp(&Card::from('2')), Greater);
        assert_eq!(Card::from('9').cmp(&Card::from('2')), Greater);
        assert_eq!(Card::from('K').cmp(&Card::from('A')), Less);
    }

    #[test]
    fn test_hand_ord() {
        assert_eq!(Hand::from("K3K39").cmp(&Hand::from("K3K39")), Equal);
        assert_eq!(Hand::from("KKK39").cmp(&Hand::from("K3K39")), Greater);
        assert_eq!(Hand::from("KKK39").cmp(&Hand::from("KKKK9")), Less);
        assert_eq!(Hand::from("AA88K").cmp(&Hand::from("AA882")), Greater);

        // full house
        assert_eq!(Hand::from("KKK88").cmp(&Hand::from("AAA42")), Greater);

        // example 33332 > 2AAAA
        assert_eq!(Hand::from("33332").cmp(&Hand::from("2AAAA")), Greater);

        // example 77888 > 77788
        assert_eq!(Hand::from("77888").cmp(&Hand::from("77788")), Greater);

        // double pair
        assert_eq!(Hand::from("22388").cmp(&Hand::from("22325")), Less);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, None);
    }
}
