use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use mygrid::{
    direction::{DOWN, LEFT, RIGHT, UP},
    grid::Grid,
    point::Point,
};

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::new_from_str(input, &|c| c);

    let mut q = VecDeque::new();
    q.push_back((HashSet::with_capacity(100), Point::new(0, 1), DOWN, 0));
    let target = Point::new_usize(grid.rows() - 1, grid.cols() - 2);

    let mut max_len = 0;
    while let Some((mut visited, p, dir, len)) = q.pop_front() {
        if visited.contains(&p) || !grid.is_in_bounds(p) || grid[p] == '#' {
            continue;
        }
        visited.insert(p);

        match grid[p] {
            '#' => {}
            '>' => q.push_back((visited.clone(), p + RIGHT, RIGHT, len + 1)),
            'v' => q.push_back((visited.clone(), p + DOWN, DOWN, len + 1)),
            '<' => q.push_back((visited.clone(), p + LEFT, LEFT, len + 1)),
            '^' => q.push_back((visited.clone(), p + UP, UP, len + 1)),
            '.' if p == target => max_len = max_len.max(len),
            '.' => {
                q.push_back((visited.clone(), p + dir, dir, len + 1));
                let dir_c = dir.rotate_clockwise();
                let dir_cc = dir.rotate_counterclockwise();
                q.push_back((visited.clone(), p + dir_cc, dir_cc, len + 1));
                q.push_back((visited.clone(), p + dir_c, dir_c, len + 1));
            }
            _ => unreachable!(),
        }
    }
    Some(max_len)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::new_from_str(input, &|c| c);

    let mut q = VecDeque::new();
    q.push_back((HashSet::new(), Point::new(0, 1), DOWN, 0, 0));
    let target = Point::new_usize(grid.rows() - 1, grid.cols() - 2);

    // let mut max_len_per_fork = HashMap::new();

    let mut max_len = 0;
    while let Some((mut visited, mut p, mut dir, mut len, forks)) = q.pop_front() {
        let mut branches;
        // advance as far as possible before branching
        loop {
            branches = [dir, dir.rotate_clockwise(), dir.rotate_counterclockwise()]
                .iter()
                .map(|&d| (d, p + d))
                .filter(|&(_, p)| grid.is_in_bounds(p))
                .filter(|&(_, p)| grid[p] != '#')
                .filter(|&(_, p)| !visited.contains(&p))
                .collect_vec();

            if branches.len() != 1 {
                break;
            }
            if p == target {
                max_len = max_len.max(len);
                break;
            }
            dir = branches[0].0;
            p = branches[0].1;
            len += 1;
        }

        // match max_len_per_fork.get(&p) {
        //     Some(&(ml_so_far, forks_so_far)) if forks == forks_so_far && len < ml_so_far => {
        //         continue
        //     }
        //     _ => max_len_per_fork.insert(p, (len, forks)),
        // };

        visited.insert(p);

        match grid[p] {
            '#' => {}
            '.' if p == target => max_len = max_len.max(len),
            _ if branches.is_empty() => {}
            '.' | '>' | 'v' | '<' | '^' => {
                // visit depth first to keep memory usage down
                branches[0..1].iter().for_each(|&(d, p)| {
                    q.push_front((visited.clone(), p, d, len + 1, forks + 1));
                });
                branches[1..].iter().for_each(|&(d, p)| {
                    q.push_back((visited.clone(), p, d, len + 1, forks + 1));
                });
            }
            _ => unreachable!(),
        }
    }
    Some(max_len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(154));
    }
}
