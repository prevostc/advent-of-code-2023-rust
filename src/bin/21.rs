use std::collections::{HashSet, VecDeque};

use mygrid::{direction::ORTHOGONAL, grid::Grid};

advent_of_code::solution!(21);

pub fn get_counts(input: &str, target_steps: u64) -> (u64, u64, u64, u64) {
    let (grid, start_pos) = Grid::new_from_str_capture_start(input, &|c| c, &|c| c == 'S');

    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    q.push_back((start_pos, 0));
    let mut total_odd = 0;
    let mut total_even = 0;
    let mut odd_corners = 0;
    let mut even_corners = 0;

    while let Some((pos, steps)) = q.pop_front() {
        if steps % 2 == 0 {
            total_even += 1;
            if steps > 65 {
                even_corners += 1;
            }
        } else {
            total_odd += 1;
            if steps > 65 {
                odd_corners += 1;
            }
        }

        for dir in ORTHOGONAL {
            let next = pos + dir;

            if grid.is_in_bounds(next) && grid[next] != '#' && !visited.contains(&next) {
                q.push_back((next, steps + 1));
                visited.insert(next);
            }
        }
    }

    (total_even, total_odd, even_corners, odd_corners)
}

fn solve_part2(input: &str, target_steps: u64) -> u64 {
    let (even_full, odd_full, even_corners, odd_corners) = get_counts(input, target_steps);
    let n = 202300;

    (n + 1) * (n + 1) * odd_full + n * n * even_full - (n + 1) * odd_corners + n * even_corners
}

pub fn part_one(input: &str) -> Option<u64> {
    let (even, _, _, _) = get_counts(input, 64);
    Some(even)
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve_part2(input, 26501365))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let (result, _, _, _) =
            get_counts(&advent_of_code::template::read_file("examples", DAY, 1), 6);
        assert_eq!(result, 16);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(16));
    }
}
