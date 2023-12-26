use std::collections::{HashMap, HashSet, VecDeque};

use bitvec::BitVec64;
use mygrid::{
    direction::{DOWN, LEFT, ORTHOGONAL, RIGHT, UP},
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
    let mut fork_points = HashSet::new();
    let start_point = Point::new(0, 1);
    let target = Point::new_usize(grid.rows() - 1, grid.cols() - 2);
    fork_points.insert(start_point);
    fork_points.insert(target);

    for (p, c) in grid.iter_item_and_position() {
        match c {
            '#' => {}
            '.' | '>' | 'v' | '<' | '^' => {
                let branch_count = ORTHOGONAL
                    .iter()
                    .map(|&d| (d, p + d))
                    .filter(|&(_, p)| grid.is_in_bounds(p))
                    .filter(|&(_, p)| grid[p] != '#')
                    .count();
                if branch_count > 2 {
                    fork_points.insert(p);
                }
            }
            _ => unreachable!(),
        }
    }

    let mut graph = HashMap::new();
    for fork_point in fork_points.iter() {
        let mut q = VecDeque::new();
        q.push_back((*fork_point, 0));
        let mut visited = HashSet::new();
        while let Some((p, len)) = q.pop_front() {
            if visited.contains(&p) || !grid.is_in_bounds(p) || grid[p] == '#' {
                continue;
            }
            visited.insert(p);

            if p != *fork_point && fork_points.contains(&p) {
                graph
                    .entry(*fork_point)
                    .or_insert_with(Vec::new)
                    .push((p, len));
                continue;
            }

            match grid[p] {
                '#' => {}
                '.' | '>' | 'v' | '<' | '^' => {
                    q.push_back((p + RIGHT, len + 1));
                    q.push_back((p + DOWN, len + 1));
                    q.push_back((p + LEFT, len + 1));
                    q.push_back((p + UP, len + 1));
                }
                _ => unreachable!(),
            }
        }
    }

    // print digraph
    // println!("digraph {{");
    // for (p, edges) in graph.iter() {
    //     for &(p2, len) in edges {
    //         // remove duplicate edges
    //         if p2.line < p.line || (p2.line == p.line && p2.column < p.column) {
    //             continue;
    //         }
    //         println!("  \"{:?}\" -> \"{:?}\" [label=\"{}\"];", p, p2, len);
    //     }
    // }
    // println!("}}");

    // println!("graph: {:?}", graph);

    // we are now in a DAG, because of the "no-backtracking" rule
    // so finding the longest path is equivalent to finding the shortest path
    // in the same DAG where each edge has negative weight -len
    // let mut neg_graph = HashMap::new();
    // for (p, edges) in graph.iter() {
    //     for &(p2, len) in edges {
    //         neg_graph
    //             .entry(*p)
    //             .or_insert_with(Vec::new)
    //             .push((p2, -(len as i32)));
    //     }
    // }

    // just dfs to find the longest path
    let mut max_len = 0;
    let mut q = VecDeque::new();
    let points = fork_points.iter().copied().collect::<Vec<_>>();
    let point_idx = points
        .iter()
        .enumerate()
        .map(|(i, p)| (*p, i))
        .collect::<HashMap<_, _>>();
    let base_visited = BitVec64::from_size(points.len() as u8);
    q.push_back((base_visited.clone(), start_point, 0));
    while let Some((visited, pos, len)) = q.pop_front() {
        if pos == target {
            max_len = max_len.max(len);
            continue;
        }

        for (n, d) in graph.get(&pos).unwrap() {
            let n_idx = point_idx[n];
            if visited[n_idx] {
                continue;
            }
            let mut new_visited = visited.clone();
            new_visited.set(n_idx, true);
            q.push_back((new_visited, *n, len + d));
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
