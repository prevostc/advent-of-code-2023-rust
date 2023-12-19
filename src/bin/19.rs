use regex::Regex;
use std::cmp;
use std::collections::{HashMap, HashSet, VecDeque};
advent_of_code::solution!(19);

#[derive(Debug)]
enum Action<'a> {
    Accept,
    Reject,
    Goto(&'a str),
}

#[derive(Debug)]
struct Parts(u64, u64, u64, u64);

#[derive(Debug)]
enum Op {
    Gt(u64),
    Eq(u64),
    Lt(u64),
    True,
}

fn parse(input: &str) -> (HashMap<&str, Vec<(u64, Op, Action<'_>)>>, Vec<Parts>) {
    let (workflows_str, ratings_str) = input.split_once("\n\n").unwrap();

    let mut workflows = HashMap::new();
    let workflow_re = Regex::new(r"(?<key>.+)\{(?<rules>.+)\}").unwrap();
    let rule_re = Regex::new(r"(?<left>.+)(?<op>(<|>|=))(?<right>.+):(?<action>.+)").unwrap();
    for line in workflows_str.lines() {
        let caps = workflow_re.captures(line).unwrap();
        let key = caps.name("key").unwrap().as_str();
        let rules = caps.name("rules").unwrap().as_str();

        let mut rule_vec = Vec::new();
        for rule_str in rules.split(",") {
            let caps = rule_re.captures(rule_str);
            if let None = caps {
                let rule = match rule_str {
                    "A" => (0, Op::True, Action::Accept),
                    "R" => (0, Op::True, Action::Reject),
                    g => (0, Op::True, Action::Goto(g)),
                };
                rule_vec.push(rule);
                continue;
            }
            let caps = caps.unwrap();
            let left_str = caps.name("left").unwrap().as_str();
            let op_str = caps.name("op").unwrap().as_str();
            let right_str = caps.name("right").unwrap().as_str();
            let action_str = caps.name("action").unwrap().as_str();

            let idx = match left_str {
                "x" => 0,
                "m" => 1,
                "a" => 2,
                "s" => 3,
                _ => panic!("Unknown register {}", left_str),
            };

            let right = right_str.parse::<u64>().unwrap();
            let op = match op_str {
                ">" => Op::Gt(right),
                "=" => Op::Eq(right),
                "<" => Op::Lt(right),
                _ => panic!("Unknown op {}", op_str),
            };

            let action = match action_str {
                "A" => Action::Accept,
                "R" => Action::Reject,
                g => Action::Goto(g),
            };

            let rule = (idx, op, action);
            rule_vec.push(rule);
        }
        workflows.insert(key, rule_vec);
    }

    let mut parts = Vec::new();
    let rating_re = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
    for rating_str in ratings_str.lines() {
        let caps = rating_re.captures(rating_str).unwrap();

        let x = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
        let m = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();
        let a = caps.get(3).unwrap().as_str().parse::<u64>().unwrap();
        let s = caps.get(4).unwrap().as_str().parse::<u64>().unwrap();

        let part = Parts(x, m, a, s);
        parts.push(part);
    }

    (workflows, parts)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (workflows, parts) = parse(input);
    //println!("{:?}", workflows);
    //println!("{:?}", parts);

    let mut total = 0;
    for part in parts {
        let mut pos = "in";
        let mut visited = HashSet::with_capacity(workflows.len());

        //println!("======= Starting {:?}", part);
        loop {
            //println!("{}", pos);
            if visited.contains(&pos) {
                break;
            }
            visited.insert(pos);
            let rules = workflows.get(pos).unwrap();
            for rule in rules {
                let (idx, op, action) = rule;
                let val = match idx {
                    0 => part.0,
                    1 => part.1,
                    2 => part.2,
                    3 => part.3,
                    _ => panic!("Unknown register {}", idx),
                };

                let decision = match op {
                    Op::Gt(right) => val > *right,
                    Op::Eq(right) => val == *right,
                    Op::Lt(right) => val < *right,
                    Op::True => true,
                };

                if decision {
                    match action {
                        Action::Accept => {
                            total += part.0 + part.1 + part.2 + part.3;
                            //println!("ACCEPTED {} {:?}", total, part);
                            break;
                        }
                        Action::Reject => {
                            //println!("REJECTED {:?}", part);
                            break;
                        }
                        Action::Goto(g) => {
                            pos = g;
                            break;
                        }
                    }
                }
            }
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (workflows, _) = parse(input);

    let mut q = VecDeque::new();
    q.push_back(("in", vec![(1, 4000), (1, 4000), (1, 4000), (1, 4000)]));

    let mut sum_possible = 0;

    while let Some(state) = q.pop_front() {
        let (pos, ranges) = state;
        let rules = workflows.get(pos).unwrap();

        let mut branches = VecDeque::new();
        branches.push_back((0, ranges));

        while let Some((rule_idx, ranges)) = branches.pop_front() {
            let (part_idx, op, action) = &rules[rule_idx];

            let (min, max) = ranges[*part_idx as usize];
            let next_branches_decision = match op {
                Op::Gt(right) => {
                    vec![
                        (false, (min, cmp::min(*right, max))),
                        (true, (cmp::max(*right + 1, min), max)),
                    ]
                }
                Op::Eq(right) => {
                    if *right < min || *right > max {
                        vec![]
                    } else {
                        vec![
                            (true, (*right, *right)),
                            (false, (min, cmp::min(*right - 1, max))),
                            (false, (cmp::max(*right + 1, min), max)),
                        ]
                    }
                }
                Op::Lt(right) => {
                    vec![
                        (true, (min, cmp::min(*right - 1, max))),
                        (false, (cmp::max(*right, min), max)),
                    ]
                }
                Op::True => vec![(true, (min, max))],
            };

            for (decision, (min, max)) in next_branches_decision {
                if min > max {
                    continue;
                }

                if !decision {
                    let mut new_ranges = ranges.clone();
                    new_ranges[*part_idx as usize] = (min, max);
                    branches.push_back((rule_idx + 1, new_ranges));
                    continue;
                }
                match action {
                    Action::Accept => {
                        let mut new_ranges = ranges.clone();
                        new_ranges[*part_idx as usize] = (min, max);
                        sum_possible += (new_ranges[0].1 - new_ranges[0].0 + 1)
                            * (new_ranges[1].1 - new_ranges[1].0 + 1)
                            * (new_ranges[2].1 - new_ranges[2].0 + 1)
                            * (new_ranges[3].1 - new_ranges[3].0 + 1);
                    }
                    Action::Reject => {}
                    Action::Goto(g) => {
                        let mut new_ranges = ranges.clone();
                        new_ranges[*part_idx as usize] = (min, max);
                        q.push_back((g, new_ranges))
                    }
                }
            }
        }
    }
    Some(sum_possible)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(167409079868000));
    }
}
