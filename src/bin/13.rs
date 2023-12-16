//use bitmatrix::*;
//use bitvec::prelude::*;
use bitvec::BitVec64;
use itertools::Itertools;
use std::fmt::{Display, Formatter, Result};

advent_of_code::solution!(13);

fn detect_best_mirror_position(data: &Vec<BitVec64>, smudges: i32) -> Option<(i32, i32)> {
    // position, size
    let mut best_mirror = None;
    let right_bound = (data.len() - 1) as i32;
    for i in 0..right_bound {
        // just consume both sides until they don't match
        // or we hit the end
        let mut left = i;
        let mut right = left + 1;
        let mut iter_max = None;
        let mut left_smudges = smudges;
        while left >= 0 && right <= right_bound {
            // count smudges between the 2
            let differences = (data[left as usize] ^ data[right as usize]).count_ones();
            if differences > left_smudges as u32 {
                break;
            }
            left_smudges -= differences as i32;
            if left_smudges < 0 {
                break;
            }

            // that only count if we reached a border
            if left == 0 || right == right_bound {
                let size = right - left + 1;
                let position = i + 1;
                iter_max = Some((position, size));
            }
            left -= 1;
            right += 1;
        }

        if left_smudges != 0 {
            continue;
        }

        match (iter_max, best_mirror) {
            (Some((pos, size)), Some((_, best_size))) if size > best_size => {
                best_mirror = Some((pos, size));
            }
            (Some((pos, size)), None) => {
                best_mirror = Some((pos, size));
            }
            _ => {}
        }
    }
    best_mirror
}

struct Lake {
    rows: Vec<BitVec64>,
    cols: Vec<BitVec64>,
}
impl Display for Lake {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "\n=== rows ===")?;
        self.rows
            .iter()
            .for_each(|row| row.as_fmt('#', '.').fmt(f).unwrap());
        writeln!(f, "\n=== cols ===")?;
        self.cols
            .iter()
            .for_each(|col| col.as_fmt('#', '.').fmt(f).unwrap());
        Ok(())
    }
}

impl Lake {
    fn new(p: &str) -> Self {
        let rows = p
            .lines()
            .map(|line| BitVec64::from_str(line, '#', '.'))
            .collect_vec();

        let width = rows[0].len();

        let cols = (0..width)
            .map(|c| rows.iter().map(|row| row[c]).collect::<BitVec64>())
            .collect::<Vec<_>>();

        return Self { rows, cols };
    }

    fn solve(&self, smudges: i32) -> (Option<i32>, Option<i32>) {
        let best_horizontal = detect_best_mirror_position(&self.cols, smudges);
        let best_vertical = detect_best_mirror_position(&self.rows, smudges);
        // only keep the best half
        match (best_horizontal, best_vertical) {
            (Some((hp, hs)), Some((_, vs))) if hs >= vs => (Some(hp), None),
            (Some((_, hs)), Some((vp, vs))) if hs < vs => (None, Some(vp)),
            (Some((hp, _)), None) => (Some(hp), None),
            (None, Some((vp, _))) => (None, Some(vp)),
            _ => (None, None),
        }
    }
}

pub fn solve(input: &str, smudges: i32) -> Option<i64> {
    let res = input
        .split("\n\n")
        .map(|p| p.trim_end_matches("\n")) // remove that pesky last newline
        .map(Lake::new)
        .map(|l| l.solve(smudges))
        .fold((Some(0), Some(0)), |(mha, mva), (mhb, mvb)| {
            (
                mha.or(Some(0))
                    .and_then(|ha| mhb.or(Some(0)).map(|hb| ha + hb)),
                mva.or(Some(0))
                    .and_then(|va| mvb.or(Some(0)).map(|vb| va + vb)),
            )
        });
    match res {
        (Some(h), Some(v)) => Some((h + 100 * v) as i64),
        _ => None,
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    solve(input, 0)
}

pub fn part_two(input: &str) -> Option<i64> {
    solve(input, 1)
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(400));
    }

    #[test]
    fn test_best_mirror_position_1() {
        let s = "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.";
        let lake = Lake::new(s);
        assert_eq!(detect_best_mirror_position(&lake.cols, 1), None);
        assert_eq!(detect_best_mirror_position(&lake.rows, 1), Some((3, 6)));
    }

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY, 2));
        assert_eq!(result, Some(810));
    }

    #[test]
    fn test_solve_1_0() {
        let s = "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.";
        let lake = Lake::new(s);

        let (horizontal, vertical) = lake.solve(0);
        assert_eq!(horizontal, Some(5));
        assert_eq!(vertical, None);
    }

    #[test]
    fn test_solve_2_0() {
        let s = "#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#";
        let lake = Lake::new(s);

        let (horizontal, vertical) = lake.solve(0);
        assert_eq!(horizontal, None);
        assert_eq!(vertical, Some(4));
    }

    #[test]
    fn test_best_mirror_position_0() {
        let s = "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.";
        let lake = Lake::new(s);
        assert_eq!(detect_best_mirror_position(&lake.cols, 0), Some((5, 8)));
        assert_eq!(detect_best_mirror_position(&lake.rows, 0), None);
    }

    #[test]
    fn test_solve_for_first_column() {
        let s = "..###.#..#.##\n#####.####.##\n...##.#..#.##\n...##.####.##\n###.#......#.\n###..######..\n###..........\n..##..##.#..#\n...###.##.###";
        let lake = Lake::new(s);
        println!("{}", lake);
        assert_eq!(detect_best_mirror_position(&lake.cols, 0), Some((1, 2)));
    }

    #[test]
    fn test_only_detect_full_reflections_up_to_a_border() {
        let s = "#.#.##..##.#.\n....#.##.#.#.\n####......#.#\n####......#.#\n....#.##.#.#.\n....##..##.#.\n#.#.#.##.#.#.";
        let lake = Lake::new(s);
        assert_eq!(detect_best_mirror_position(&lake.cols, 0), None);
        assert_eq!(detect_best_mirror_position(&lake.rows, 0), None);
    }

    #[test]
    fn test_detect_regression() {
        let s = ".###..###\n.##....##\n#.#....#.\n.########\n##.#..#.#\n##......#\n##..###.#\n#.#.##.#.\n.#......#\n..#....#.\n####..###\n...#..#..\n...#..#..\n####..###\n..#....#.";
        let lake = Lake::new(s);
        assert_eq!(detect_best_mirror_position(&lake.rows, 0), Some((12, 6)));
    }

    #[test]
    fn test_lake_new() {
        let s = "..#\n#..\n...";
        let lake = Lake::new(s);
        assert_eq!(lake.rows.len(), 3);
        assert_eq!(lake.rows.len(), 3);
        println!("{}", lake);
    }
}
