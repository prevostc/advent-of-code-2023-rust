use grid::*;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(17);

struct Map {
    map: Grid<u32>,
}

impl Map {
    fn new(input: &str) -> Self {
        let map = Grid::from_vec(
            input
                .chars()
                .filter(|&c| c != '\n')
                .map(|c| c.to_digit(10).unwrap())
                .collect_vec(),
            input.lines().next().unwrap().len(),
        );

        Self { map }
    }

    #[inline(always)]
    fn is_in_bounds_after_move(&self, pos: (isize, isize), dir: (isize, isize)) -> bool {
        let new_pos = (pos.0 + dir.0, pos.1 + dir.1);

        if new_pos.0 < 0 || new_pos.1 < 0 {
            return false;
        }

        if new_pos.0 as usize >= self.map.rows() || new_pos.1 as usize >= self.map.cols() {
            return false;
        }

        true
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.map.rows() {
            for c in 0..self.map.cols() {
                write!(f, "{}", self.map[(r, c)]).unwrap();
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Walker {
    pos: (isize, isize),
    dir: (isize, isize),
    blocks: u32,
}

impl Walker {
    fn new(pos: (isize, isize), dir: (isize, isize), blocks: u32) -> Self {
        Self { pos, dir, blocks }
    }
}

fn print_debug(map: &Map, path: &Vec<(isize, isize)>, walker: &Walker) {
    println!("==========≠≠≠======≠≠≠==========");
    for r in 0..map.map.rows() {
        for c in 0..map.map.cols() {
            print!("{}", map.map[(r, c)]);
        }

        print!("  ");

        for c in 0..map.map.cols() {
            let pos = (r as isize, c as isize);
            if path.contains(&pos) {
                print!("*");
            } else if pos == walker.pos {
                match walker.dir {
                    (0, 1) => print!(">"),
                    (0, -1) => print!("<"),
                    (1, 0) => print!("v"),
                    (-1, 0) => print!("^"),
                    _ => print!("?"),
                }
            } else {
                print!(".");
            }
        }

        println!("");
    }
}

pub fn solve(input: &str, min_blocks: u32, max_blocks: u32) -> Option<i32> {
    let map = Map::new(input);
    let mut visited: HashMap<Walker, u32> = HashMap::new();
    let mut queue = VecDeque::new();
    let target = (map.map.rows() as isize - 1, map.map.cols() as isize - 1);
    queue.push_back((0, Walker::new((0, 0), (0, 1), 1)));
    queue.push_back((0, Walker::new((0, 0), (1, 0), 1)));

    let mut min_heat_loss = u32::max_value();
    while let Some((heat_loss, walker)) = queue.pop_front() {
        if walker.pos == target {
            //println!("found target: {:?}", walker);
            if heat_loss < min_heat_loss {
                //println!("found new min heat loss: {}", heat_loss);
                min_heat_loss = heat_loss;
            }
            //print_debug(&map, &path, &walker);
            //println!("Heat: min:{} current:{}", min_heat_loss, heat_loss);
        }

        let forward = (walker.dir, walker.blocks + 1);
        let left = ((-walker.dir.1, walker.dir.0), 1);
        let right = ((walker.dir.1, -walker.dir.0), 1);
        let mut directions = vec![];
        if walker.blocks < min_blocks {
            directions.push(forward);
        } else {
            directions.push(left);
            directions.push(right);
            if walker.blocks < max_blocks {
                directions.push(forward);
            }
        }

        directions
            .iter()
            .filter(|(dir, _)| map.is_in_bounds_after_move(walker.pos, *dir))
            .for_each(|&(dir, new_blocks)| {
                let new_pos = (walker.pos.0 + dir.0, walker.pos.1 + dir.1);
                let new_heat_loss = heat_loss + map.map[(new_pos.0 as usize, new_pos.1 as usize)];
                let new_walker = Walker::new(new_pos, dir, new_blocks);

                if visited.contains_key(&new_walker) && new_heat_loss >= visited[&new_walker] {
                    return;
                }

                visited.insert(new_walker.clone(), new_heat_loss);
                queue.push_back((new_heat_loss, new_walker));
            });
    }
    Some(min_heat_loss as i32)
}

pub fn part_one(input: &str) -> Option<i32> {
    solve(input, 1, 3)
}

pub fn part_two(input: &str) -> Option<i32> {
    solve(input, 4, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two_ex() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, 2));
        assert_eq!(result, Some(71));
    }
}
