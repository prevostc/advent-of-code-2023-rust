use std::hash::Hash;
use std::ops::{Index, IndexMut};

use crate::point::Point;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    content: Vec<T>,
}

impl<T> Grid<T> {
    #[inline]
    pub fn new(width: usize, height: usize, default: T) -> Self
    where
        T: Copy,
    {
        Self {
            width,
            height,
            content: vec![default; width * height],
        }
    }

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
    pub fn new_from_str_capture_start(
        input: &str,
        map_char: &dyn Fn(char) -> T,
        is_start: &dyn Fn(T) -> bool,
    ) -> (Self, Point)
    where
        T: From<char> + Copy,
    {
        let width = input.lines().next().unwrap().len();

        let mut content = Vec::new();
        let mut start = None;
        for (i, c) in input.chars().filter(|&c| c != '\n').enumerate() {
            let t = map_char(c);
            if is_start(t) {
                start = Some(Point::new_usize(i / width, i % width));
            }
            content.push(t);
        }

        let height = content.len() / width;

        (
            Self {
                width,
                height,
                content,
            },
            start.unwrap(),
        )
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

    #[inline]
    pub fn row(&self, row: usize) -> &[T] {
        &self.content[(row * self.width)..((row + 1) * self.width)]
    }

    #[inline]
    pub fn resize(&mut self, width: usize, height: usize, default: T)
    where
        T: Copy,
    {
        // we can't use vec.resize because it would just truncate the last elements on shrink
        let mut new_content = vec![default; width * height];
        for line in 0..height {
            for column in 0..width {
                let point = Point::new(line as isize, column as isize);
                if self.is_in_bounds(point) {
                    new_content[(line * width) + column] = self[point];
                } else {
                    new_content[(line * width) + column] = default;
                }
            }
        }
        self.content = new_content;
        self.width = width;
        self.height = height;
    }

    #[inline]
    pub fn resize_to_max_point(&mut self, point: Point, default: T)
    where
        T: Copy,
    {
        let width = (point.column + 1) as usize;
        let height = (point.line + 1) as usize;
        self.resize(width, height, default);
    }

    #[inline]
    pub fn clamp(&mut self, min: Point, max: Point, default: T)
    where
        T: Copy,
    {
        let width = (max.column - min.column + 1) as usize;
        let height = (max.line - min.line + 1) as usize;
        let mut new_content = vec![default; width * height];
        for line in min.line..=max.line {
            for column in min.column..=max.column {
                let point = Point::new(line, column);
                let index = (((line - min.line) * width as isize) + (column - min.column)) as usize;
                if self.is_in_bounds(point) {
                    new_content[index] = self[point];
                } else {
                    new_content[index] = default;
                }
            }
        }
        self.content = new_content;
        self.width = width;
        self.height = height;
    }
}

impl<T> Grid<T> {
    #[inline]
    pub fn is_in_bounds(&self, point: Point) -> bool {
        point.column >= 0
            && point.column < (self.width as isize)
            && point.line >= 0
            && point.line < (self.height as isize)
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.content.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.content.iter_mut()
    }

    #[inline]
    pub fn iter_rows(&self) -> impl Iterator<Item = &[T]> {
        self.content.chunks(self.width)
    }

    #[inline]
    pub fn iter_rows_mut(&mut self) -> impl Iterator<Item = &mut [T]> {
        self.content.chunks_mut(self.width)
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
    pub fn test_grid_row() {
        let grid = Grid::new_from_str("123\n456\n789", &|c| c);
        assert_eq!(grid.row(0), &['1', '2', '3'][..]);
        assert_eq!(grid.row(1), &['4', '5', '6'][..]);
        assert_eq!(grid.row(2), &['7', '8', '9'][..]);
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
    pub fn test_grid_new_from_str_with_start_pos() {
        let (grid, start) =
            Grid::new_from_str_capture_start("123\n456\n789", &|c| c, &|c| c == '5');
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
        assert_eq!(start, Point::new(1, 1));
    }

    #[test]
    pub fn test_iter() {
        let grid = Grid::new_from_str("123\n456\n789", &|c| c);
        let mut iter = grid.iter();
        assert_eq!(iter.next(), Some(&'1'));
        assert_eq!(iter.next(), Some(&'2'));
        assert_eq!(iter.next(), Some(&'3'));
        assert_eq!(iter.next(), Some(&'4'));
        assert_eq!(iter.next(), Some(&'5'));
        assert_eq!(iter.next(), Some(&'6'));
        assert_eq!(iter.next(), Some(&'7'));
        assert_eq!(iter.next(), Some(&'8'));
        assert_eq!(iter.next(), Some(&'9'));
        assert_eq!(iter.next(), None);
    }

    #[test]
    pub fn test_iter_mut() {
        let mut grid = Grid::new_from_str("123\n456\n789", &|c| c);
        let mut iter = grid.iter_mut();
        assert_eq!(iter.next(), Some(&mut '1'));
        assert_eq!(iter.next(), Some(&mut '2'));
        assert_eq!(iter.next(), Some(&mut '3'));
        assert_eq!(iter.next(), Some(&mut '4'));
        assert_eq!(iter.next(), Some(&mut '5'));
        assert_eq!(iter.next(), Some(&mut '6'));
        assert_eq!(iter.next(), Some(&mut '7'));
        assert_eq!(iter.next(), Some(&mut '8'));
        assert_eq!(iter.next(), Some(&mut '9'));
        assert_eq!(iter.next(), None);
    }

    #[test]
    pub fn test_iter_rows() {
        let grid = Grid::new_from_str("123\n456\n789", &|c| c);
        let mut iter = grid.iter_rows();
        assert_eq!(iter.next(), Some(&['1', '2', '3'][..]));
        assert_eq!(iter.next(), Some(&['4', '5', '6'][..]));
        assert_eq!(iter.next(), Some(&['7', '8', '9'][..]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    pub fn test_iter_rows_mut() {
        let mut grid = Grid::new_from_str("123\n456\n789", &|c| c);
        let mut iter = grid.iter_rows_mut();
        assert_eq!(iter.next(), Some(&mut ['1', '2', '3'][..]));
        assert_eq!(iter.next(), Some(&mut ['4', '5', '6'][..]));
        assert_eq!(iter.next(), Some(&mut ['7', '8', '9'][..]));
        assert_eq!(iter.next(), None);
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

    #[test]
    pub fn test_grid_resize_shrink() {
        let mut grid = Grid::new_from_str("123\n456\n789", &|c| c);
        grid.resize(2, 2, '0');
        assert_eq!(grid.width, 2);
        assert_eq!(grid.height, 2);
        assert_eq!(grid.content.len(), 4);
        assert_eq!(grid.content[0], '1');
        assert_eq!(grid.content[1], '2');
        assert_eq!(grid.content[2], '4');
        assert_eq!(grid.content[3], '5');
    }

    #[test]
    pub fn test_grid_resize_grow() {
        let mut grid = Grid::new_from_str("123\n456\n789", &|c| c);
        grid.resize(4, 4, '0');
        assert_eq!(grid.width, 4);
        assert_eq!(grid.height, 4);
        assert_eq!(grid.content.len(), 16);
        assert_eq!(grid.content[0], '1');
        assert_eq!(grid.content[1], '2');
        assert_eq!(grid.content[2], '3');
        assert_eq!(grid.content[3], '0');
        assert_eq!(grid.content[4], '4');
        assert_eq!(grid.content[5], '5');
        assert_eq!(grid.content[6], '6');
        assert_eq!(grid.content[7], '0');
        assert_eq!(grid.content[8], '7');
        assert_eq!(grid.content[9], '8');
        assert_eq!(grid.content[10], '9');
        assert_eq!(grid.content[11], '0');
        assert_eq!(grid.content[12], '0');
        assert_eq!(grid.content[13], '0');
        assert_eq!(grid.content[14], '0');
        assert_eq!(grid.content[15], '0');
    }

    #[test]
    pub fn test_grid_resize_to_max_point() {
        let mut grid = Grid::new_from_str("123\n456\n789", &|c| c);
        grid.resize_to_max_point(Point::new(3, 3), '0');
        assert_eq!(grid.width, 4);
        assert_eq!(grid.height, 4);
        assert_eq!(grid.content.len(), 16);
        assert_eq!(grid.content[0], '1');
        assert_eq!(grid.content[1], '2');
        assert_eq!(grid.content[2], '3');
        assert_eq!(grid.content[3], '0');
        assert_eq!(grid.content[4], '4');
        assert_eq!(grid.content[5], '5');
        assert_eq!(grid.content[6], '6');
        assert_eq!(grid.content[7], '0');
        assert_eq!(grid.content[8], '7');
        assert_eq!(grid.content[9], '8');
        assert_eq!(grid.content[10], '9');
        assert_eq!(grid.content[11], '0');
        assert_eq!(grid.content[12], '0');
        assert_eq!(grid.content[13], '0');
        assert_eq!(grid.content[14], '0');
        assert_eq!(grid.content[15], '0');
    }

    #[test]
    pub fn test_grid_clamp() {
        let mut grid = Grid::new_from_str("123\n456\n789", &|c| c);
        grid.clamp(Point::new(1, 1), Point::new(2, 2), '0');
        assert_eq!(grid.width, 2);
        assert_eq!(grid.height, 2);
        assert_eq!(grid.content.len(), 4);
        assert_eq!(grid.content[0], '5');
        assert_eq!(grid.content[1], '6');
        assert_eq!(grid.content[2], '8');
        assert_eq!(grid.content[3], '9');
    }
}
