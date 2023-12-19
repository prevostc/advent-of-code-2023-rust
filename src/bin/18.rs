use mygrid::{Direction, Grid, Point, DOWN, LEFT, ORTHOGONAL, RIGHT, UP};
use std::collections::VecDeque;

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<i64> {
    // parse instructions
    let instructions = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let dir: Direction = parts.next().unwrap().into();
            let dist: i64 = parts.next().unwrap().parse().unwrap();
            (dir, dist)
        })
        .collect::<Vec<_>>();

    // transform to grid
    let mut grid = Grid::new(1000, 1000, b'.');
    let mut pos = Point::new(500, 500);
    grid[pos] = b'#';
    let mut min_pos = pos;
    let mut max_pos = pos;
    for (dir, dist) in instructions {
        for _ in 0..dist {
            pos = pos + dir;
            if !grid.is_in_bounds(pos) {
                continue;
            }
            grid[pos] = b'#';
            max_pos = max_pos.max(&pos);
            min_pos = min_pos.min(&pos);
        }
    }
    grid.clamp(min_pos + LEFT + UP, max_pos + RIGHT + DOWN, b'.');
    //println!("{}", grid.to_fmt(|_, c| format!("{}", *c as char)));

    // fill into with bfs
    let mut queue = VecDeque::new();
    queue.push_back(Point::new(0, 0)); // fill the exterior
    while let Some(pos) = queue.pop_front() {
        if !grid.is_in_bounds(pos) {
            continue;
        }
        if grid[pos] == b'#' || grid[pos] == b'~' {
            continue;
        }
        grid[pos] = b'~';
        for dir in ORTHOGONAL {
            queue.push_back(pos + dir);
        }
    }
    //println!("{}", grid.to_fmt(|_, c| format!("{}", *c as char)));

    // count # and .
    let mut count = 0;
    for c in grid.iter() {
        if *c == b'#' || *c == b'.' {
            count += 1;
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<i64> {
    // parse instructions
    let instructions = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let rgb = parts.nth(2).unwrap();
            let dist = i32::from_str_radix(&rgb[2..rgb.len() - 2], 16).unwrap();
            let dir = match rgb.chars().nth(7).unwrap() {
                '0' => RIGHT,
                '1' => DOWN,
                '2' => LEFT,
                '3' => UP,
                _ => unreachable!(),
            };
            (dir, dist)
        })
        .collect::<Vec<_>>();
    //println!("{:?}", instructions);

    // https://www.mathopenref.com/coordpolygonarea.html
    let mut pos = Point::new(0, 0);
    let mut agg: i64 = 0;
    let mut border = 0;
    for (dir, dist) in instructions {
        let new_pos = pos + (dir * dist);
        agg += pos.column as i64 * new_pos.line as i64 - pos.line as i64 * new_pos.column as i64;
        pos = new_pos;
        border += dist as i64;
    }
    Some((agg.abs() / 2) + (border / 2) + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(952408144115));
    }
}
