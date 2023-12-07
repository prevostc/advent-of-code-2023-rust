use itertools::EitherOrBoth::{self, Both, Left, Right};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;

advent_of_code::solution!(7);

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Hash, Ord)]
enum Card {
    Value(i32),
}

type Hand = [Card; 5];

#[derive(Debug, Clone)]
struct Game {
    hand: Hand,
    bets: u32,
    card_counts: Vec<(Card, usize)>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let ref char_value = "AKQJT98765432"
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| (c, Card::Value(i as i32)))
        .collect::<HashMap<_, _>>();

    // maybe we can cheat a bit
    let ordered_games = input
        .lines()
        .map(|l| {
            l.split_once(" ")
                .map(|(cards, b)| {
                    let mut hand = [Card::Value(0); 5];
                    cards
                        .chars()
                        .flat_map(|c| char_value.get(&c))
                        .enumerate()
                        .for_each(|(i, c)| {
                            hand[i] = (*c).clone();
                        });
                    let bets = b.parse::<u32>().unwrap();
                    let card_counts: Vec<(Card, usize)> = hand
                        .into_iter()
                        .counts()
                        .into_iter()
                        .sorted_by(|(card_a, count_a), (card_b, count_b)| {
                            match count_b.cmp(count_a) {
                                Ordering::Equal => card_b.cmp(card_a),
                                o => o,
                            }
                        })
                        .map(|(c, v)| (c, v))
                        .collect::<Vec<_>>();

                    Game {
                        hand,
                        bets,
                        card_counts,
                    }
                })
                .unwrap()
        })
        .sorted_by(|a, b| {
            for v in a
                .card_counts
                .iter()
                .zip_longest(&b.card_counts)
                .map(|e| match e {
                    EitherOrBoth::Both((_, a), (_, b)) if *a != *b => a.cmp(b),
                    EitherOrBoth::Both((Card::Value(a), _), (Card::Value(b), _)) if a != b => {
                        a.cmp(b)
                    }
                    EitherOrBoth::Both(_, _) => Ordering::Equal,
                    EitherOrBoth::Left(_) => Ordering::Greater,
                    EitherOrBoth::Right(_) => Ordering::Less,
                })
            {
                match v {
                    Ordering::Equal => {}
                    _ => return v,
                }
            }
            return Ordering::Equal;
        })
        .collect::<Vec<_>>();

    let res = ordered_games
        .iter()
        .enumerate()
        .map(|(i, g)| (i as u32 + 1) * g.bets)
        .sum();
    dbg!(ordered_games);
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

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
