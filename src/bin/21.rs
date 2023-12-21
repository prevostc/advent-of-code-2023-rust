use std::collections::{HashSet, VecDeque};

use mygrid::{direction::ORTHOGONAL, grid::Grid, point::Point};

advent_of_code::solution!(21);

pub fn solve(input: &str, target_steps: u64) -> Option<u64> {
    let (grid, start_pos) = Grid::new_from_str_capture_start(input, &|c| c, &|c| c == 'S');

    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    q.push_back((start_pos, 0));
    let mut total_matching = 0;

    while let Some((pos, steps)) = q.pop_front() {
        if !grid.is_in_bounds(pos)
            || visited.contains(&pos)
            || grid[pos] == '#'
            || steps > target_steps
        {
            continue;
        }
        visited.insert(pos);

        if steps % 2 == 0 {
            total_matching += 1;
        }

        for dir in ORTHOGONAL {
            q.push_back((pos + dir, steps + 1));
        }
    }

    Some(total_matching)
}

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, 6)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY, 1), 6);
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(16));
    }
}
