// grid library
// contains everything related to grids, points and directions
// heavily inspired by the amazing maneatingape repo, from which I learned a lot, plz see:
// https://github.com/maneatingape/advent-of-code-rust/blob/main/src/util/point.rs

use std::hash::{Hash, Hasher};

use crate::direction::Direction;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Point {
    pub line: isize,
    pub column: isize,
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
    pub const fn infinite_grid_to_real_grid(
        &self,
        real_grid_lines: usize,
        real_grid_columns: usize,
    ) -> Self {
        // account for negative values as well
        let line = (self.line % real_grid_lines as isize + real_grid_lines as isize)
            % real_grid_lines as isize;
        let column = (self.column % real_grid_columns as isize + real_grid_columns as isize)
            % real_grid_columns as isize;
        Point::new(line, column)
    }

    #[inline]
    pub fn apply_direction(&self, direction: Direction) -> Self {
        Point::new(
            self.line + direction.vertical,
            self.column + direction.horizontal,
        )
    }

    #[inline]
    pub fn max(&self, other: &Point) -> Self {
        Point::new(self.line.max(other.line), self.column.max(other.column))
    }

    #[inline]
    pub fn min(&self, other: &Point) -> Self {
        Point::new(self.line.min(other.line), self.column.min(other.column))
    }
}

impl Hash for Point {
    #[inline]
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write_isize(self.line);
        hasher.write_isize(self.column);
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(l:{}, c:{})", self.line, self.column)
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
    pub fn test_point_max() {
        let point = Point::new(1, 2);
        let other = Point::new(3, 1);
        let max = point.max(&other);
        assert_eq!(max.line, 3);
        assert_eq!(max.column, 2);
    }

    #[test]
    pub fn test_point_min() {
        let point = Point::new(1, 2);
        let other = Point::new(3, 1);
        let min = point.min(&other);
        assert_eq!(min.line, 1);
        assert_eq!(min.column, 1);
    }

    #[test]
    pub fn test_infinite_grid_to_real_grid() {
        let point = Point::new(45, 20);
        let real_grid_lines = 3;
        let real_grid_columns = 4;
        let real_grid_point = point.infinite_grid_to_real_grid(real_grid_lines, real_grid_columns);
        assert_eq!(real_grid_point.line, 0);
        assert_eq!(real_grid_point.column, 0);
    }

    #[test]
    pub fn test_infinite_grid_to_real_grid_negative() {
        let point = Point::new(-5, -8);
        let real_grid_lines = 3;
        let real_grid_columns = 4;
        let real_grid_point = point.infinite_grid_to_real_grid(real_grid_lines, real_grid_columns);
        assert_eq!(real_grid_point.line, 1);
        assert_eq!(real_grid_point.column, 0);
    }
}
