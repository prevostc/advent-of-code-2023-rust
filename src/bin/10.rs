use grid::Grid;
use std::cmp;

advent_of_code::solution!(10);

trait UsizeAddI32 {
    fn add(&self, v: i32) -> Option<usize>;
}
impl UsizeAddI32 for usize {
    fn add(&self, i: i32) -> Option<usize> {
        if i.is_negative() {
            self.checked_sub(i.wrapping_abs() as usize)
        } else {
            self.checked_add(i as usize)
        }
    }
}

type Position = (usize, usize);
type PositionDiff = (i32, i32);

trait PositionAddDiff {
    fn add(&self, d: PositionDiff) -> Option<Position>;
}
impl PositionAddDiff for Position {
    fn add(&self, (dl, dc): PositionDiff) -> Option<Position> {
        match (self.0.add(dl), self.1.add(dc)) {
            (Some(line), Some(col)) => Some((line, col)),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Tile {
    Pipe(char, PositionDiff, PositionDiff),
    Nothing,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '|' => Tile::Pipe('|', (-1, 0), (1, 0)),
            '-' => Tile::Pipe('-', (0, -1), (0, 1)),
            'L' => Tile::Pipe('L', (-1, 0), (0, 1)),
            'J' => Tile::Pipe('J', (-1, 0), (0, -1)),
            '7' => Tile::Pipe('7', (1, 0), (0, -1)),
            'F' => Tile::Pipe('F', (1, 0), (0, 1)),
            '.' => Tile::Nothing,
            _ => panic!("Invalid char '{c}'"),
        }
    }

    fn can_receive(&self, pos: Position, from_pos: Position) -> bool {
        match self {
            Tile::Pipe(_, a, b) => {
                pos.add(*a).map_or(false, |np| np.eq(&from_pos))
                    || pos.add(*b).map_or(false, |np| np.eq(&from_pos))
            }
            _ => false,
        }
    }

    fn is_pipe_compatible(&self, pos: Position, neigh: Tile, neigh_pos: Position) -> bool {
        let self_can_receive = self.can_receive(pos, neigh_pos);
        let neighbour_can_receive = neigh.can_receive(neigh_pos, pos);
        return self_can_receive && neighbour_can_receive;
    }
}

#[derive(Debug)]
struct Field {
    data: Grid<Tile>,
    sheep_starts_at: Position,
}

impl Field {
    fn from_str(input: &str) -> Self {
        let line_len = input.lines().next().unwrap().len();
        let mut sheep_idx = 0;
        let elems = input
            .chars()
            .filter(|c| *c != '\n')
            .enumerate()
            // could detect that but
            .map(|(i, c)| {
                if c == 'S' {
                    sheep_idx = i;
                    'J'
                } else {
                    c
                }
            })
            .map(Tile::from_char)
            .collect::<Vec<_>>();
        let sheep_line = sheep_idx / line_len;
        let sheep_col = sheep_idx % line_len;

        Self {
            data: Grid::from_vec(elems, line_len),
            sheep_starts_at: (
                sheep_line.try_into().unwrap(),
                sheep_col.try_into().unwrap(),
            ),
        }
    }

    fn next_compatible_adjacent(&self, pos: Position, from: Position) -> Option<Position> {
        vec![(-1, 0), (0, 1), (1, 0), (0, -1)]
            .iter()
            .flat_map(|n| pos.add(*n))
            .filter(|n| n.0 < self.data.rows() && n.1 < self.data.cols())
            .filter(move |n| !n.eq(&from))
            .filter(|n| {
                let compatible = match (self.data[pos], self.data[*n]) {
                    (Tile::Nothing, _) | (_, Tile::Nothing) => false,
                    (to, tn) => to.is_pipe_compatible(pos, tn, *n),
                };
                compatible
            })
            .take(1)
            .next()
    }
}

pub fn part(input: &str, p1: bool) -> Option<i32> {
    let field = Field::from_str(input);

    // compute the visited parts
    let mut visited: Grid<bool> = Grid::from_vec(
        field.data.iter().map(|_| false).collect::<Vec<_>>(),
        field.data.cols(),
    );

    let origin = field.sheep_starts_at;
    let mut cur_pos = origin;
    let mut coming_from = origin;
    let mut distance = 0;
    while let Some(next_adjacent) = field.next_compatible_adjacent(cur_pos, coming_from) {
        // stop if we are back at the origin
        if next_adjacent == origin {
            break;
        }

        visited[cur_pos] = true;
        coming_from = cur_pos;
        cur_pos = next_adjacent;
        distance += 1;
    }

    if p1 {
        return Some((distance / 2) + 1);
    }

    // now for each dot, see if we can reach the border with an odd number of visited nodes
    let mid_point = field.data.cols() / 2;
    let res = field
        .data
        .indexed_iter()
        .filter_map(|(coo, _)| if visited[coo] { None } else { Some(coo) })
        .filter(|coo| {
            let (line, col) = *coo;
            let range = if col > mid_point {
                0..col
            } else {
                (col + 1)..field.data.cols()
            };
            let counts = range.map(|c| (line, c)).filter(|coo| visited[*coo]).fold(
                (0, 0, 0),
                |(b, f7, lj), coo| match field.data[coo] {
                    Tile::Pipe('F' | '7', _, _) => (b, f7 + 1, lj),
                    Tile::Pipe('L' | 'J', _, _) => (b, f7, lj + 1),
                    Tile::Pipe('|', _, _) => (b + 1, f7, lj),
                    _ => (b, f7, lj),
                },
            );

            let jumps = counts.0 + cmp::min(counts.1, counts.2);
            jumps % 2 == 1
        })
        .count();

    return Some(res as i32);
}

pub fn part_one(input: &str) -> Option<i32> {
    part(input, true)
}
pub fn part_two(input: &str) -> Option<i32> {
    part(input, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, 3));
        assert_eq!(result, Some(4));
    }
    #[test]
    fn test_part_two_5() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, 5));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_one_simple() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_longer() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY, 2));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two_4() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, 4));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_usize_add_neg() {
        let us: usize = 10;
        assert_eq!(us.add(3), Some(13));
        assert_eq!(us.add(-3), Some(7));
        assert_eq!(us.add(0), Some(10));
        assert_eq!(us.add(-10), Some(0));
        assert_eq!(us.add(-11), None);
    }

    #[test]
    fn test_position_add_diff() {
        let pos: Position = (10, 10);
        assert_eq!(pos.add((10, 10)), Some((20, 20)));
        assert_eq!(pos.add((-10, -5)), Some((0, 5)));
        assert_eq!(pos.add((-11, -5)), None);
        assert_eq!(pos.add((-5, -11)), None);
    }

    #[test]
    fn test_compatible_pipe() {
        // F-
        let p1 = Tile::from_char('F');
        let p2 = Tile::from_char('-');
        assert_eq!(p1.is_pipe_compatible((0, 0), p2, (0, 1)), true);
        // -F
        assert_eq!(p1.is_pipe_compatible((0, 1), p2, (0, 0)), false);
        // F -
        assert_eq!(p1.is_pipe_compatible((0, 0), p2, (0, 2)), false);
    }
}
