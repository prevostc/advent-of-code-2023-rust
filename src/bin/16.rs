use grid::*;
use itertools::Itertools;
use rayon::prelude::*;
advent_of_code::solution!(16);

#[derive(Debug, Clone, PartialEq)]
struct Beam {
    pos: (isize, isize),
    dir: (isize, isize),
}

impl Beam {
    fn new(pos: (isize, isize), dir: (isize, isize)) -> Self {
        Self { pos, dir }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Map {
    grid: Grid<char>,
    visited_horizontally: Grid<bool>,
    visited_vertically: Grid<bool>,
    beams: Vec<Beam>,
}
impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.grid.rows() {
            for c in 0..self.grid.cols() {
                write!(f, "{}", self.grid[(r, c)]).unwrap();
            }
            write!(f, "  ")?;
            for c in 0..self.grid.cols() {
                let beam = self
                    .beams
                    .iter()
                    .find(|b| b.pos == (r as isize, c as isize));
                match beam {
                    Some(b) => match b.dir {
                        (0, 1) => write!(f, ">")?,
                        (0, -1) => write!(f, "<")?,
                        (1, 0) => write!(f, "v")?,
                        (-1, 0) => write!(f, "^")?,
                        _ => write!(f, "?")?,
                    },
                    None => {
                        let vh = self.visited_horizontally[(r, c)];
                        let vv = self.visited_vertically[(r, c)];
                        if vh && vv {
                            write!(f, "#")?;
                        } else if vh {
                            write!(f, "=")?;
                        } else if vv {
                            write!(f, "I")?;
                        } else {
                            write!(f, ".")?;
                        }
                    }
                }
            }

            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Map {
    fn new(input: &str) -> Self {
        let cols = input.lines().next().unwrap().len();
        let chars = input.chars().filter(|c| !c.eq(&'\n')).collect_vec();
        let bools_h = chars.iter().map(|_| false).collect_vec();
        let bools_v = chars.iter().map(|_| false).collect_vec();
        Self {
            grid: Grid::from_vec(chars, cols),
            visited_horizontally: Grid::from_vec(bools_h, cols),
            visited_vertically: Grid::from_vec(bools_v, cols),
            beams: vec![Beam {
                pos: (0, -1),
                dir: (0, 1),
            }],
        }
    }

    fn count_energized(&self) -> u32 {
        let mut count = 0;
        for r in 0..self.grid.rows() {
            for c in 0..self.grid.cols() {
                if self.visited_horizontally[(r, c)] || self.visited_vertically[(r, c)] {
                    count += 1;
                }
            }
        }

        return count;
    }

    fn reset(&mut self, initial_beam: Beam) {
        self.beams = vec![initial_beam];
        for r in 0..self.grid.rows() {
            for c in 0..self.grid.cols() {
                self.visited_horizontally[(r, c)] = false;
                self.visited_vertically[(r, c)] = false;
            }
        }
    }

    fn init(&mut self, initial_beam: Beam) {
        self.beams = vec![initial_beam];
    }

    fn step_beams(&mut self) {
        let mut nb = Vec::with_capacity(self.beams.len() + 1);
        for b in self.beams.iter_mut() {
            let npos: (isize, isize) = (b.pos.0 + b.dir.0, b.pos.1 + b.dir.1);
            if npos.0 < 0
                || npos.1 < 0
                || npos.0 as usize >= self.grid.rows()
                || npos.1 as usize >= self.grid.cols()
            {
                continue;
            }

            let (r, c) = (npos.0 as usize, npos.1 as usize);
            let is_visited;
            if b.dir.0 == 0 {
                is_visited = self.visited_horizontally[(r, c)];
                self.visited_horizontally[(r, c)] = true;
            } else {
                is_visited = self.visited_vertically[(r, c)];
                self.visited_vertically[(r, c)] = true;
            };

            match (is_visited, self.grid[(r, c)]) {
                (true, '.' | '|' | '-') => {} // already visited and we'll learn nothing new
                (false, '.') => {
                    nb.push(Beam::new(npos, b.dir));
                }
                (false, '|') => {
                    if b.dir.1 == 0 {
                        nb.push(Beam::new(npos, b.dir));
                    } else {
                        nb.push(Beam::new(npos, (-1, 0)));
                        nb.push(Beam::new(npos, (1, 0)));
                    }
                }
                (false, '-') => {
                    if b.dir.0 == 0 {
                        nb.push(Beam::new(npos, b.dir));
                    } else {
                        nb.push(Beam::new(npos, (0, -1)));
                        nb.push(Beam::new(npos, (0, 1)));
                    }
                }
                (_, '/') => {
                    nb.push(Beam::new(npos, (-b.dir.1, -b.dir.0)));
                }
                (_, '\\') => {
                    nb.push(Beam::new(npos, (b.dir.1, b.dir.0)));
                }
                _ => panic!("unknown char"),
            }
        }
        self.beams = nb;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = Map::new(input);

    while map.beams.len() > 0 {
        map.step_beams();
    }

    Some(map.count_energized())
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = Map::new(input);

    let top_row = (0..map.grid.cols()).map(|c| Beam::new((-1, c as isize), (1, 0)));
    let bottom_row =
        (0..map.grid.cols()).map(|c| Beam::new((map.grid.rows() as isize, c as isize), (-1, 0)));
    let left_col = (0..map.grid.rows()).map(|r| Beam::new((r as isize, -1), (0, 1)));
    let right_col =
        (0..map.grid.rows()).map(|r| Beam::new((r as isize, map.grid.cols() as isize), (0, -1)));
    let all_edge_beams = top_row
        .chain(bottom_row)
        .chain(left_col)
        .chain(right_col)
        .collect_vec();

    let max_energy = all_edge_beams
        .par_iter()
        .map(|start_beam| {
            let mut map_copy = map.clone();
            map_copy.init(start_beam.clone());
            while map_copy.beams.len() > 0 {
                map_copy.step_beams();
            }
            map_copy.count_energized()
        })
        .max();

    max_energy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(51));
    }
}
