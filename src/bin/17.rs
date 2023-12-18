use mygrid::{Direction, Grid, Point, DOWN, ORTHOGONAL, RIGHT};
use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(17);

struct Map {
    map: Grid<u32>,
}

impl Map {
    fn new(input: &str) -> Self {
        let map = Grid::new_from_str(input, &|c: char| c.to_digit(10).unwrap());
        Self { map }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Walker {
    pos: Point,
    dir: Direction,
    blocks: u32,
}

impl Walker {
    fn new(pos: Point, dir: Direction, blocks: u32) -> Self {
        Self { pos, dir, blocks }
    }
}

fn print_debug(map: &Map, path: &Vec<Point>, walker: &Walker) {
    println!("==========≠≠≠======≠≠≠==========");

    println!("{}", map.map.to_fmt(|_, n| format!("{}", n)));
    println!(
        "{}",
        map.map.to_fmt(|pos, _| {
            if path.contains(&pos) {
                "*".to_owned()
            } else if pos == walker.pos {
                format!("{}", walker.dir)
            } else {
                ".".to_owned()
            }
        })
    );
}

pub fn solve(input: &str, min_blocks: u32, max_blocks: u32) -> Option<i32> {
    let map = Map::new(input);
    let mut visited: HashMap<Walker, u32> = HashMap::new();
    let mut queue = VecDeque::new();
    let target = Point::new_usize(map.map.rows() - 1, map.map.cols() - 1);
    let start = Point::new(0, 0);
    queue.push_back((0, Walker::new(start, RIGHT, 1)));
    queue.push_back((0, Walker::new(start, DOWN, 1)));

    let mut min_heat_loss = u32::max_value();
    while let Some((heat_loss, walker)) = queue.pop_front() {
        if walker.pos == target {
            if heat_loss < min_heat_loss {
                min_heat_loss = heat_loss;
            }
        }

        let forward = (walker.dir, walker.blocks + 1);
        let left = (walker.dir.rotate_counterclockwise(), 1);
        let right = (walker.dir.rotate_clockwise(), 1);
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
            .filter(|(dir, _)| map.map.is_in_bounds(walker.pos))
            .for_each(|(dir, new_blocks)| {
                let new_pos = walker.pos + *dir;
                let new_heat_loss = heat_loss + map.map[walker.pos];
                let new_walker = Walker::new(new_pos, *dir, *new_blocks);

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
