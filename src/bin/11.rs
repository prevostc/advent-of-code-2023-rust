advent_of_code::solution!(11);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos(i32, i32);

impl Pos {
    fn distance(&self, other: &Self) -> i64 {
        ((self.0 - other.0).abs() + (self.1 - other.1).abs()) as i64
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Map {
    galaxies: Vec<Pos>,
    max_x: i32,
    max_y: i32,
}

impl Map {
    fn new(input: &str) -> Self {
        let max_x = input.lines().next().unwrap().len() as i32;
        let max_y = input.lines().count() as i32;

        let galaxies: Vec<Pos> = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '#')
                    .map(move |(x, _)| Pos(x as i32, y as i32))
            })
            .collect();
        Self {
            galaxies,
            max_x,
            max_y,
        }
    }

    fn expand(&mut self, n: i32) {
        // add 1 to each galaxy's x and y for each empty space between it and 0,0
        // slow but simple
        let empty_x: Vec<i32> = (0..self.max_x)
            .filter(|x| !self.galaxies.iter().any(|p| p.0 == *x))
            .collect();
        let empty_y: Vec<i32> = (0..self.max_y)
            .filter(|y| !self.galaxies.iter().any(|p| p.1 == *y))
            .collect();

        // could be way faster with some sort of sorting index
        for galaxy in &mut self.galaxies {
            galaxy.0 += (empty_x.iter().filter(|x| **x < galaxy.0).count() as i32) * n;
            galaxy.1 += (empty_y.iter().filter(|y| **y < galaxy.1).count() as i32) * n;
        }

        // double the empty x space between galaxies
        self.max_x = self.max_x + (empty_x.len() as i32) * n;
        self.max_y = self.max_y + (empty_y.len() as i32) * n;
    }

    fn print(&self) {
        for y in 0..self.max_y {
            for x in 0..self.max_x {
                if self.galaxies.contains(&Pos(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

pub fn solve(input: &str, n: i32) -> Option<i64> {
    let mut map = Map::new(input);
    map.expand(n);

    let mut sum_dist: i64 = 0;
    for (i, g1) in map.galaxies.iter().enumerate() {
        for g2 in &map.galaxies[(i + 1)..] {
            sum_dist += g1.distance(g2);
        }
    }

    Some(sum_dist)
}

fn part_one(input: &str) -> Option<i64> {
    solve(input, 1)
}
fn part_two(input: &str) -> Option<i64> {
    solve(input, 1000000 - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY, 1), 1);
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two_10() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY, 1), 9);
        assert_eq!(result, Some(1030));
    }

    #[test]
    fn test_part_two_100() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY, 1), 99);
        assert_eq!(result, Some(8410));
    }

    #[test]
    fn test_expand_galaxies() {
        let mut map = Map::new(
            r#"#..
...
..#"#,
        );
        assert_eq!(map.galaxies, vec![Pos(0, 0), Pos(2, 2)]);
        map.expand(1);
        assert_eq!(map.galaxies, vec![Pos(0, 0), Pos(3, 3)]);
    }

    #[test]
    fn test_expand_galaxies_multiple() {
        let mut map = Map::new(
            r#"#..
...
..#"#,
        );
        assert_eq!(map.galaxies, vec![Pos(0, 0), Pos(2, 2)]);
        map.expand(100);
        assert_eq!(map.galaxies, vec![Pos(0, 0), Pos(102, 102)]);
    }

    #[test]
    fn test_pos_distance() {
        assert_eq!(Pos(0, 0).distance(&Pos(0, 0)), 0);
        assert_eq!(Pos(0, 0).distance(&Pos(1, 0)), 1);
        assert_eq!(Pos(0, 0).distance(&Pos(0, 1)), 1);
        assert_eq!(Pos(0, 0).distance(&Pos(1, 1)), 2);
        assert_eq!(Pos(0, 0).distance(&Pos(2, 2)), 4);
        assert_eq!(Pos(0, 0).distance(&Pos(3, 3)), 6);
    }
}
