use std::hash::{Hash, Hasher};
use std::ops::{Add, Mul};

use crate::point::Point;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Direction {
    pub vertical: isize,
    pub horizontal: isize,
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

impl From<&str> for Direction {
    #[inline]
    fn from(value: &str) -> Self {
        Direction::from(value.chars().next().unwrap())
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

impl Mul<isize> for Direction {
    type Output = Direction;

    #[inline]
    fn mul(self, rhs: isize) -> Self::Output {
        Direction::new(self.vertical * rhs, self.horizontal * rhs)
    }
}

impl Mul<i32> for Direction {
    type Output = Direction;

    #[inline]
    fn mul(self, rhs: i32) -> Self::Output {
        self * (rhs as isize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point::Point;

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
    pub fn test_direction_mult() {
        let direction = Direction::new(2, 0);
        let new_direction: Direction = direction * 2;
        assert_eq!(new_direction.vertical, 4);
        assert_eq!(new_direction.horizontal, 0);
    }
}
