// grid library
// contains everything related to grids, points and directions
// heavily inspired by the amazing maneatingape repo, from which I learned a lot, plz see:
// https://github.com/maneatingape/advent-of-code-rust/blob/main/src/util/point.rs

use std::hash::{Hash, Hasher};
use std::ops::Add;
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Point {
    line: isize,
    column: isize,
}

impl Point {
    #[inline]
    pub const fn new_i32(line: i32, column: i32) -> Self {
        assert!(line >= 0);
        assert!(column >= 0);
        Point::new(line as isize, column as isize)
    }

    #[inline]
    pub const fn new_usize(line: usize, column: usize) -> Self {
        Point::new(line as isize, column as isize)
    }

    #[inline]
    pub const fn new(line: isize, column: isize) -> Self {
        Point { line, column }
    }

    #[inline]
    pub fn apply_direction(&self, direction: Direction) -> Self {
        Point::new(
            self.line + direction.vertical,
            self.column + direction.horizontal,
        )
    }
}

impl Hash for Point {
    #[inline]
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write_isize(self.line);
        hasher.write_isize(self.column);
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Direction {
    vertical: isize,
    horizontal: isize,
}

pub const UP: Direction = Direction::new(-1, 0);
pub const DOWN: Direction = Direction::new(1, 0);
pub const LEFT: Direction = Direction::new(0, -1);
pub const RIGHT: Direction = Direction::new(0, 1);
pub const ORTHOGONAL: [Direction; 4] = [UP, DOWN, LEFT, RIGHT];
pub const ALL_AROUND: [Direction; 8] = [
    UP,
    UP.add_direction(&RIGHT),
    RIGHT,
    RIGHT.add_direction(&DOWN),
    DOWN,
    DOWN.add_direction(&LEFT),
    LEFT,
    LEFT.add_direction(&UP),
];

impl Direction {
    #[inline]
    pub const fn new(vertical: isize, horizontal: isize) -> Self {
        assert!(vertical.abs() <= 1);
        assert!(horizontal.abs() <= 1);
        Direction {
            vertical,
            horizontal,
        }
    }

    #[inline]
    pub const fn new_i32(vertical: i32, horizontal: i32) -> Self {
        Direction::new(vertical as isize, horizontal as isize)
    }

    #[inline]
    pub fn rotate_clockwise(&self) -> Self {
        Direction::new(self.horizontal, -self.vertical)
    }

    #[inline]
    pub fn rotate_clockwise_mut(&mut self) {
        (self.horizontal, self.vertical) = (-self.vertical, self.horizontal);
    }

    #[inline]
    pub fn rotate_counterclockwise(&self) -> Self {
        Direction::new(-self.horizontal, self.vertical)
    }

    #[inline]
    pub fn rotate_counterclockwise_mut(&mut self) {
        (self.horizontal, self.vertical) = (self.vertical, -self.horizontal);
    }

    #[inline]
    pub fn reverse(&self) -> Self {
        Direction::new(-self.vertical, -self.horizontal)
    }

    #[inline]
    pub fn reverse_mut(&mut self) {
        (self.horizontal, self.vertical) = (-self.horizontal, -self.vertical);
    }

    #[inline]
    pub fn is_opposite(&self, other: &Direction) -> bool {
        self.vertical == -other.vertical && self.horizontal == -other.horizontal
    }

    #[inline]
    pub fn is_orthogonal(&self, other: &Direction) -> bool {
        self.vertical == 0 && other.vertical == 0 || self.horizontal == 0 && other.horizontal == 0
    }

    #[inline]
    pub const fn add_direction(&self, other: &Direction) -> Self {
        Direction::new(
            self.vertical + other.vertical,
            self.horizontal + other.horizontal,
        )
    }

    #[inline]
    pub fn add_direction_mut(&mut self, other: &Direction) {
        self.vertical += other.vertical;
        self.horizontal += other.horizontal;
        assert!(self.vertical.abs() <= 1);
        assert!(self.horizontal.abs() <= 1);
    }
}

impl Hash for Direction {
    #[inline]
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write_isize(self.vertical);
        hasher.write_isize(self.horizontal);
    }
}

impl From<char> for Direction {
    #[inline]
    fn from(value: char) -> Self {
        match value {
            '^' | 'U' => UP,
            'v' | 'D' => DOWN,
            '<' | 'L' => LEFT,
            '>' | 'R' => RIGHT,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            UP => write!(f, "^"),
            DOWN => write!(f, "v"),
            LEFT => write!(f, "<"),
            RIGHT => write!(f, ">"),
            _ => unreachable!(),
        }
    }
}

impl Add<Direction> for Point {
    type Output = Point;

    #[inline]
    fn add(self, rhs: Direction) -> Self::Output {
        self.apply_direction(rhs)
    }
}

impl Add<Direction> for Direction {
    type Output = Direction;

    #[inline]
    fn add(self, rhs: Direction) -> Self::Output {
        Direction::new(
            self.vertical + rhs.vertical,
            self.horizontal + rhs.horizontal,
        )
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    content: Vec<T>,
}

impl<T> Grid<T> {
    #[inline]
    pub fn new_from_str(input: &str, map_char: &dyn Fn(char) -> T) -> Self
    where
        T: From<char>,
    {
        let width = input.lines().next().unwrap().len();
        let content = input
            .chars()
            .filter(|&c| c != '\n')
            .map(map_char)
            .collect::<Vec<_>>();

        let height = content.len() / width;

        Self {
            width,
            height,
            content,
        }
    }

    #[inline]
    pub fn from_vec(content: Vec<T>, width: usize) -> Self {
        let height = content.len() / width;

        Self {
            width,
            height,
            content,
        }
    }

    #[inline]
    pub fn cols(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn rows(&self) -> usize {
        self.height
    }
}

impl<T: Copy + PartialEq> Grid<T> {
    #[inline]
    pub fn is_in_bounds(&self, point: Point) -> bool {
        point.column >= 0
            && point.column < (self.width as isize)
            && point.line >= 0
            && point.line < (self.height as isize)
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, point: Point) -> &Self::Output {
        &self.content[((point.line as usize) * self.width) + (point.column as usize)]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        &mut self.content[((point.line as usize) * self.width) + (point.column as usize)]
    }
}

// print the grid using a user parameterized function
impl<T> Grid<T> {
    #[inline]
    pub fn to_fmt<F>(&self, f: F) -> Grid<String>
    where
        F: Fn(Point, &T) -> String,
    {
        let mut grid = Grid::from_vec(vec!["x".to_owned(); self.content.len()], self.width);
        for line in 0..self.height {
            for column in 0..self.width {
                let point = Point::new(line as isize, column as isize);
                grid[point] = f(point, &self[point]);
            }
        }
        grid
    }
}

impl std::fmt::Display for Grid<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.height {
            for c in 0..self.width {
                let point = Point::new_usize(r, c);
                write!(f, "{}", self[point])?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_point() {
        let point = Point::new(1, 2);
        assert_eq!(point.line, 1);
        assert_eq!(point.column, 2);
    }

    #[test]
    pub fn test_point_new_i32() {
        let point = Point::new_i32(1, 2);
        assert_eq!(point.line, 1);
        assert_eq!(point.column, 2);
    }

    #[test]
    pub fn test_direction() {
        let direction = Direction::new(1, 0);
        assert_eq!(direction.vertical, 1);
        assert_eq!(direction.horizontal, 0);
    }

    #[test]
    pub fn test_point_apply_direction() {
        let point = Point::new(1, 2);
        let direction = Direction::new(1, 0);
        let new_point = point.apply_direction(direction);
        assert_eq!(new_point.line, 2);
        assert_eq!(new_point.column, 2);

        let new_point = point + direction;
        assert_eq!(new_point.line, 2);
        assert_eq!(new_point.column, 2);
    }

    #[test]
    pub fn test_direction_rotate() {
        let direction = Direction::new(1, 0);
        let new_direction = direction.rotate_clockwise();
        assert_eq!(new_direction.vertical, 0);
        assert_eq!(new_direction.horizontal, -1);

        let new_direction = direction.rotate_counterclockwise();
        assert_eq!(new_direction.vertical, 0);
        assert_eq!(new_direction.horizontal, 1);

        let mut direction = Direction::new(1, 0);
        direction.rotate_clockwise_mut();
        assert_eq!(direction.vertical, 0);
        assert_eq!(direction.horizontal, -1);

        let mut direction = Direction::new(1, 0);
        direction.rotate_counterclockwise_mut();
        assert_eq!(direction.vertical, 0);
        assert_eq!(direction.horizontal, 1);
    }

    #[test]
    pub fn test_direction_from_char() {
        let direction = Direction::from('U');
        assert_eq!(direction.vertical, -1);
        assert_eq!(direction.horizontal, 0);

        let direction = Direction::from('D');
        assert_eq!(direction.vertical, 1);
        assert_eq!(direction.horizontal, 0);

        let direction = Direction::from('L');
        assert_eq!(direction.vertical, 0);
        assert_eq!(direction.horizontal, -1);

        let direction = Direction::from('R');
        assert_eq!(direction.vertical, 0);
        assert_eq!(direction.horizontal, 1);
    }

    #[test]
    pub fn test_grid() {
        let grid = Grid::new_from_str("123\n456\n789", &|c| c);
        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 3);
        assert_eq!(grid.content.len(), 9);
        assert_eq!(grid.content[0], '1');
        assert_eq!(grid.content[1], '2');
        assert_eq!(grid.content[2], '3');
        assert_eq!(grid.content[3], '4');
        assert_eq!(grid.content[4], '5');
        assert_eq!(grid.content[5], '6');
        assert_eq!(grid.content[6], '7');
        assert_eq!(grid.content[7], '8');
        assert_eq!(grid.content[8], '9');
    }

    #[test]
    pub fn test_grid_contains() {
        let grid = Grid::new_from_str("123\n456\n789", &|c| c);
        assert!(grid.is_in_bounds(Point::new(0, 0)));
        assert!(grid.is_in_bounds(Point::new(0, 1)));
        assert!(grid.is_in_bounds(Point::new(0, 2)));
        assert!(grid.is_in_bounds(Point::new(1, 0)));
        assert!(grid.is_in_bounds(Point::new(1, 1)));
        assert!(grid.is_in_bounds(Point::new(1, 2)));
        assert!(grid.is_in_bounds(Point::new(2, 0)));
        assert!(grid.is_in_bounds(Point::new(2, 1)));
        assert!(grid.is_in_bounds(Point::new(2, 2)));
        assert!(!grid.is_in_bounds(Point::new(-1, 0)));
        assert!(!grid.is_in_bounds(Point::new(0, -1)));
        assert!(!grid.is_in_bounds(Point::new(3, 0)));
        assert!(!grid.is_in_bounds(Point::new(0, 3)));
    }

    #[test]
    pub fn test_grid_index() {
        let grid = Grid::new_from_str("123\n456\n789", &|c| c);
        assert_eq!(grid[Point::new(0, 0)], '1');
        assert_eq!(grid[Point::new(0, 1)], '2');
        assert_eq!(grid[Point::new(0, 2)], '3');
        assert_eq!(grid[Point::new(1, 0)], '4');
        assert_eq!(grid[Point::new(1, 1)], '5');
        assert_eq!(grid[Point::new(1, 2)], '6');
        assert_eq!(grid[Point::new(2, 0)], '7');
        assert_eq!(grid[Point::new(2, 1)], '8');
        assert_eq!(grid[Point::new(2, 2)], '9');
    }

    #[test]
    pub fn test_grid_index_mut() {
        let mut grid = Grid::new_from_str("123\n456\n789", &|c| c);
        grid[Point::new(0, 0)] = 'a';
        grid[Point::new(0, 1)] = 'b';
        grid[Point::new(0, 2)] = 'c';
        grid[Point::new(1, 0)] = 'd';
        grid[Point::new(1, 1)] = 'e';
        grid[Point::new(1, 2)] = 'f';
        grid[Point::new(2, 0)] = 'g';
        grid[Point::new(2, 1)] = 'h';
        grid[Point::new(2, 2)] = 'i';
        assert_eq!(grid[Point::new(0, 0)], 'a');
        assert_eq!(grid[Point::new(0, 1)], 'b');
        assert_eq!(grid[Point::new(0, 2)], 'c');
        assert_eq!(grid[Point::new(1, 0)], 'd');
        assert_eq!(grid[Point::new(1, 1)], 'e');
        assert_eq!(grid[Point::new(1, 2)], 'f');
        assert_eq!(grid[Point::new(2, 0)], 'g');
        assert_eq!(grid[Point::new(2, 1)], 'h');
        assert_eq!(grid[Point::new(2, 2)], 'i');
    }

    #[test]
    pub fn test_grid_from_vec() {
        let grid = Grid::from_vec(vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'], 3);
        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 3);
        assert_eq!(grid.content.len(), 9);
        assert_eq!(grid.content[0], '1');
        assert_eq!(grid.content[1], '2');
        assert_eq!(grid.content[2], '3');
        assert_eq!(grid.content[3], '4');
        assert_eq!(grid.content[4], '5');
        assert_eq!(grid.content[5], '6');
        assert_eq!(grid.content[6], '7');
        assert_eq!(grid.content[7], '8');
        assert_eq!(grid.content[8], '9');
    }

    #[test]
    pub fn test_grid_fmt() {
        let grid = Grid::new_from_str("123\n456\n789", &|c| c);
        let grid_fmt = grid.to_fmt(|_, c| format!("{}", c));
        assert_eq!(format!("{}", grid_fmt), "123\n456\n789\n");
        let grid_fmt = grid.to_fmt(|p, _| format!("{}", p.line));
        assert_eq!(format!("{}", grid_fmt), "000\n111\n222\n");
        let grid_fmt = grid.to_fmt(|p, _| format!("{}", p.column));
        assert_eq!(format!("{}", grid_fmt), "012\n012\n012\n");

        // test we can still use grid
        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 3);
        assert_eq!(grid.content.len(), 9);
    }
}
