use std::collections::VecDeque;

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
                let can_receive_from_a = pos.add(*a).map_or(false, |np| np.eq(&from_pos));
                let can_receive_from_b = pos.add(*b).map_or(false, |np| np.eq(&from_pos));
                can_receive_from_a || can_receive_from_b
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

    fn adjacent(&self, pos: Position) -> Vec<Position> {
        vec![(-1, 0), (0, 1), (1, 0), (0, -1)]
            .iter()
            .flat_map(|n| pos.add(*n))
            .filter(|n| n.0 < self.data.rows() && n.1 < self.data.cols())
            .map(|n| n.clone())
            .collect()
    }
}

pub fn part(input: &str, p1: bool) -> Option<i32> {
    let field = Field::from_str(input);

    // compute the visited parts
    let mut visited: Grid<bool> = Grid::from_vec(
        field.data.iter().map(|_| false).collect::<Vec<_>>(),
        field.data.cols(),
    );

    let mut queue = VecDeque::new();
    queue.push_front((field.sheep_starts_at, 0));

    let mut max_dst = 0;
    while let Some((pos, dst)) = queue.pop_back() {
        if visited[pos] {
            continue;
        }
        visited[pos] = true;

        // push compatible neighbours
        for n in field.adjacent(pos) {
            let compatible = match (pos, n, field.data[pos], field.data[n]) {
                (_, _, _, Tile::Nothing) => false,
                (_, _, Tile::Nothing, _) => false,
                (_, _, to, tn) => to.is_pipe_compatible(pos, tn, n),
            };

            if compatible && !visited[n] {
                let new_dst = dst + 1;
                max_dst = cmp::max(new_dst, max_dst);
                queue.push_front((n, new_dst));
            }
        }
    }
    if p1 {
        return Some(max_dst);
    }

    // now for each dot, see if we can reach the border with an odd number of visited nodes
    let mut inside_count = 0;
    for line in 0..field.data.rows() {
        for col in 0..field.data.cols() {
            if visited[(line, col)] {
                continue;
            }

            // horizontally, F cancels 7, L cancels J
            let mut is_inside = false;
            // left
            for r in vec![(0..col), (col + 1)..field.data.cols()] {
                let mut jumps: i32 = 0;
                let mut tF7s: i32 = 0;
                let mut tJLs: i32 = 0;
                for c in r {
                    let coo = (line, c);
                    if !visited[coo] {
                        continue;
                    }
                    match field.data[coo] {
                        Tile::Pipe('F' | '7', _, _) => tF7s += 1,
                        Tile::Pipe('L' | 'J', _, _) => tJLs += 1,
                        Tile::Pipe('|', _, _) => jumps += 1,
                        _ => {}
                    }
                }
                jumps += cmp::min(tF7s, tJLs);
                if jumps % 2 == 1 {
                    is_inside = true;
                    break;
                }
            }

            for r in vec![(0..line), (line + 1)..field.data.rows()] {
                let mut jumps: i32 = 0;
                let mut tFLs: i32 = 0;
                let mut t7Js: i32 = 0;
                for l in r {
                    let coo = (l, col);
                    if !visited[coo] {
                        continue;
                    }
                    match field.data[coo] {
                        Tile::Pipe('F' | 'L', _, _) => tFLs += 1,
                        Tile::Pipe('7' | 'J', _, _) => t7Js += 1,
                        Tile::Pipe('-', _, _) => jumps += 1,
                        _ => {}
                    }
                }
                jumps += cmp::min(tFLs, t7Js);
                if jumps % 2 == 1 {
                    is_inside = true;
                    break;
                }
            }

            if is_inside {
                inside_count += 1;
            }
        }
    }
    Some(inside_count)
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
    fn test_adjacent() {
        let field = Field::from_str(
            ".....
.....
..S..
.....
.....",
        );
        assert_eq!(field.adjacent((1, 1)), vec![(0, 1), (1, 2), (2, 1), (1, 0)]);
        assert_eq!(field.adjacent((0, 0)), vec![(0, 1), (1, 0)]);
        assert_eq!(field.adjacent((4, 4)), vec![(3, 4), (4, 3)]);
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
