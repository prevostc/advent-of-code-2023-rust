use num::integer::{div_floor, gcd};
use std::collections::HashMap;

advent_of_code::solution!(8);

#[derive(Debug, Clone, Eq, Copy, Hash)]
struct NodeCode {
    code: i32,
    is_start_node: bool,
    is_end_node: bool,
}

impl NodeCode {
    fn from_str(s: &str) -> NodeCode {
        Self {
            code: i32::from_str_radix(s, 26 + 10).unwrap(), // boooh
            is_end_node: s.ends_with("Z"),
            is_start_node: s.ends_with("A"),
        }
    }
}

impl PartialEq for NodeCode {
    fn eq(&self, other: &Self) -> bool {
        self.code.eq(&other.code)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Node {
    left: NodeCode,
    right: NodeCode,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            o => panic!("Invalid direction {o}"),
        }
    }
}

struct DirectionRing {
    content: Vec<Direction>,
    cur_idx: usize,
}

impl DirectionRing {
    fn from_directions(directions: &Vec<Direction>) -> DirectionRing {
        Self {
            content: directions.clone(),
            cur_idx: 0,
        }
    }
}

impl Iterator for DirectionRing {
    type Item = Direction;
    fn next(&mut self) -> Option<Self::Item> {
        if self.content.len() == 0 {
            return None;
        }

        let elem = Some(self.content[self.cur_idx]);
        self.cur_idx += 1;
        if self.cur_idx >= self.content.len() {
            self.cur_idx = 0;
        }

        elem
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let directions = input
        .lines()
        .nth(0)
        .unwrap()
        .chars()
        .map(Direction::from_char)
        .collect::<Vec<_>>();

    let graph = input
        .lines()
        .skip(2)
        .map(|line| {
            let res = (
                NodeCode::from_str(&line[0..3]),
                Node {
                    left: NodeCode::from_str(&line[7..10]),
                    right: NodeCode::from_str(&line[12..15]),
                },
            );
            res
        })
        .collect::<HashMap<NodeCode, Node>>();

    let target = NodeCode::from_str("ZZZ");
    let ring = DirectionRing::from_directions(&directions);
    let mut cur_pos = NodeCode::from_str("AAA");
    let mut steps = 0;
    for dir in ring {
        let cur_node = graph.get(&cur_pos).unwrap();
        cur_pos = match dir {
            Direction::Left => cur_node.left,
            Direction::Right => cur_node.right,
        };
        steps += 1;
        if cur_pos.eq(&target) {
            break;
        }
    }
    Some(steps)
}

pub fn part_two(input: &str) -> Option<u64> {
    let directions = input
        .lines()
        .nth(0)
        .unwrap()
        .chars()
        .map(Direction::from_char)
        .collect::<Vec<_>>();

    let graph = input
        .lines()
        .skip(2)
        .map(|line| {
            let res = (
                NodeCode::from_str(&line[0..3]),
                Node {
                    left: NodeCode::from_str(&line[7..10]),
                    right: NodeCode::from_str(&line[12..15]),
                },
            );
            res
        })
        .collect::<HashMap<NodeCode, Node>>();

    let mut all_cur_pos = graph
        .keys()
        .filter(|c| c.is_start_node)
        .map(|c| c.clone())
        .collect::<Vec<_>>();

    let mut dst: Vec<Option<u64>> = all_cur_pos.iter().map(|_| None).collect();
    let ring = DirectionRing::from_directions(&directions);
    let mut steps = 0;
    for dir in ring {
        // advance 1 step everyone
        all_cur_pos = all_cur_pos
            .into_iter()
            .map(|cur_pos| graph.get(&cur_pos).unwrap())
            .map(|cur_node| match dir {
                Direction::Left => cur_node.left,
                Direction::Right => cur_node.right,
            })
            .collect();
        steps += 1;

        dst = dst
            .into_iter()
            .zip(&all_cur_pos)
            .map(|(d, cur_pos)| match d {
                None => {
                    if cur_pos.is_end_node {
                        Some(steps)
                    } else {
                        None
                    }
                }
                a => a,
            })
            .collect();

        if dst.iter().all(|d| d.is_some()) {
            return dst
                .iter()
                .map(|o| o.unwrap())
                .reduce(|agg, i| div_floor(agg * i, gcd(agg, i)));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ring_iter() {
        let dirs = vec![Direction::Left, Direction::Right];
        let mut ring = DirectionRing::from_directions(&dirs);

        assert_eq!(ring.next(), Some(Direction::Left));
        assert_eq!(ring.next(), Some(Direction::Right));
        assert_eq!(ring.next(), Some(Direction::Left));
        assert_eq!(ring.next(), Some(Direction::Right));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY, 2));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, 3));
        assert_eq!(result, Some(6));
    }
}
