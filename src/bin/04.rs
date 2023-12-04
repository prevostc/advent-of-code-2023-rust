use regex::Regex;
use std::collections::HashSet;
advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let re =
        Regex::new(r"^Card +(?<card_n>\d+): (?<card_numbers>[ 0-9]+) \| (?<your_numbers>[ 0-9]+)$")
            .unwrap();

    let mut total_score = 0;
    for line in input.lines() {
        let mut cards_nums = HashSet::new();
        let caps = re.captures(line).unwrap();
        let card_n = &caps["card_n"];
        let card_numbers = &caps["card_numbers"];
        let your_numbers = &caps["your_numbers"];

        for num in card_numbers.split(" ") {
            let s = num.trim_end().trim_start();
            if s.len() > 0 {
                cards_nums.insert(s);
            }
        }

        let mut card_score = 0;
        for num in your_numbers.split(" ") {
            if cards_nums.contains(num) {
                if card_score == 0 {
                    card_score = 1;
                } else {
                    card_score *= 2;
                }
            }

            println!("{}: {} {}", card_n, total_score, card_score)
        }
        total_score += card_score
    }
    Some(total_score)
}

pub fn part_two(input: &str) -> Option<i32> {
    let re =
        Regex::new(r"^Card +(?<card_n>\d+): (?<card_numbers>[ 0-9]+) \| (?<your_numbers>[ 0-9]+)$")
            .unwrap();

    let card_count = input.lines().count();
    let mut copies: Vec<i32> = vec![0; card_count];
    let mut card_matches: Vec<usize> = vec![0; card_count];

    for line in input.lines() {
        let mut cards_nums = HashSet::new();
        let caps = re.captures(line).unwrap();
        let card_n = (&caps["card_n"]).parse::<usize>().unwrap();
        let card_numbers = &caps["card_numbers"];
        let your_numbers = &caps["your_numbers"];

        copies[card_n - 1] = 1;

        for num in card_numbers.split(" ") {
            let s = num.trim_end().trim_start();
            if s.len() > 0 {
                cards_nums.insert(s);
            }
        }

        let mut card_score = 0;
        let mut match_count = 0;
        for num in your_numbers.split(" ") {
            if cards_nums.contains(num) {
                match_count += 1;
                if card_score == 0 {
                    card_score = 1;
                } else {
                    card_score *= 2;
                }
            }
        }
        card_matches[card_n - 1] = match_count;
    }

    // now resolve the card counts
    fn resolve_counts(copies: &mut Vec<i32>, card_matches: &mut Vec<usize>, card_n: usize) {
        for i in 0..card_matches[card_n - 1] {
            let idx: usize = ((card_n - 1) + i) + 1;
            copies[idx] += 1;

            resolve_counts(copies, card_matches, idx + 1);
        }
    }

    for i in 0..card_count {
        resolve_counts(&mut copies, &mut card_matches, i + 1);
    }

    let res = copies.into_iter().sum();

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(30));
    }
}
