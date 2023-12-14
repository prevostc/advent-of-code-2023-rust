use std::fmt::Display;

advent_of_code::solution!(14);

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    SquareRock,
    RoundRock,
    Empty,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Tile::SquareRock,
            'O' => Tile::RoundRock,
            '.' => Tile::Empty,
            _ => panic!("Invalid tile"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Platform {
    tiles: Vec<Vec<Tile>>,
}

impl From<&str> for Platform {
    fn from(s: &str) -> Self {
        Platform {
            tiles: s
                .lines()
                .map(|line| line.chars().map(Tile::from).collect())
                .collect(),
        }
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.tiles {
            for tile in line {
                match tile {
                    Tile::SquareRock => write!(f, "#")?,
                    Tile::RoundRock => write!(f, "O")?,
                    Tile::Empty => write!(f, ".")?,
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

// i did envision a more generic solution, but, life.
enum Direction {
    North,
    South,
    East,
    West,
}

impl Platform {
    #[inline]
    fn width(&self) -> usize {
        self.tiles[0].len()
    }

    #[inline]
    fn height(&self) -> usize {
        self.tiles.len()
    }

    #[inline]
    fn tilt_north(&mut self) {
        // work by column since we need to swap rows
        for col_idx in 0..self.width() {
            let mut free_spot_idx = None;
            for line_idx in 0..self.height() {
                let tile = &self.tiles[line_idx][col_idx];

                match (free_spot_idx, tile) {
                    (None, Tile::Empty) => {
                        free_spot_idx = Some(line_idx);
                    }
                    (_, Tile::SquareRock) => free_spot_idx = None,
                    (Some(free_idx), Tile::RoundRock) => {
                        self.tiles[free_idx][col_idx] = Tile::RoundRock;
                        self.tiles[line_idx][col_idx] = Tile::Empty;
                        free_spot_idx = Some(free_idx + 1);
                    }
                    _ => {}
                }
            }
        }
    }

    #[inline]
    fn tilt_south(&mut self) {
        // work by column since we need to swap rows
        for col_idx in 0..self.width() {
            let mut free_spot_idx = None;
            for line_idx in (0..self.height()).rev() {
                let tile = &self.tiles[line_idx][col_idx];

                match (free_spot_idx, tile) {
                    (None, Tile::Empty) => {
                        free_spot_idx = Some(line_idx);
                    }
                    (_, Tile::SquareRock) => free_spot_idx = None,
                    (Some(free_idx), Tile::RoundRock) => {
                        self.tiles[free_idx][col_idx] = Tile::RoundRock;
                        self.tiles[line_idx][col_idx] = Tile::Empty;
                        free_spot_idx = Some(free_idx - 1);
                    }
                    _ => {}
                }
            }
        }
    }

    #[inline]
    fn tilt_east(&mut self) {
        // work by column since we need to swap rows
        for line_idx in 0..self.height() {
            let mut free_spot_idx = None;
            for col_idx in (0..self.width()).rev() {
                let tile = &self.tiles[line_idx][col_idx];

                match (free_spot_idx, tile) {
                    (None, Tile::Empty) => {
                        free_spot_idx = Some(col_idx);
                    }
                    (_, Tile::SquareRock) => free_spot_idx = None,
                    (Some(free_idx), Tile::RoundRock) => {
                        self.tiles[line_idx][free_idx] = Tile::RoundRock;
                        self.tiles[line_idx][col_idx] = Tile::Empty;
                        free_spot_idx = Some(free_idx - 1);
                    }
                    _ => {}
                }
            }
        }
    }

    #[inline]
    fn tilt_west(&mut self) {
        // work by column since we need to swap rows
        for line_idx in 0..self.height() {
            let mut free_spot_idx = None;
            for col_idx in 0..self.width() {
                let tile = &self.tiles[line_idx][col_idx];

                match (free_spot_idx, tile) {
                    (None, Tile::Empty) => {
                        free_spot_idx = Some(col_idx);
                    }
                    (_, Tile::SquareRock) => free_spot_idx = None,
                    (Some(free_idx), Tile::RoundRock) => {
                        self.tiles[line_idx][free_idx] = Tile::RoundRock;
                        self.tiles[line_idx][col_idx] = Tile::Empty;
                        free_spot_idx = Some(free_idx + 1);
                    }
                    _ => {}
                }
            }
        }
    }

    #[inline]
    fn cycle(&mut self) {
        //println!("============ CYCLE BEGIN ==========\n{}", self);
        self.tilt_north();
        //println!("------------ cycled north----------\n{}", self);
        self.tilt_west();
        //println!("------------ cycled west----------\n{}", self);
        self.tilt_south();
        //println!("------------ cycled south----------\n{}", self);
        self.tilt_east();
        //println!("------------ cycled east----------\n{}", self);
        //println!("============ CYCLE END ==========\n");
    }

    fn total_load_north_beam(&self) -> u32 {
        let mut score = 0;

        for line_idx in 0..self.height() {
            for col_idx in 0..self.width() {
                let tile = &self.tiles[line_idx][col_idx];

                match tile {
                    Tile::RoundRock => score += (self.height() as u32) - (line_idx as u32),
                    _ => {}
                }
            }
        }
        score
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut platform = Platform::from(input);
    //println!("{:?}", platform);
    platform.tilt_north();
    //println!("{:?}", platform);
    Some(platform.total_load_north_beam())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut platform = Platform::from(input);
    let cycle_count = 1000000000;

    // there is most probably a loop, it's just a matter of finding it
    let states_to_keep = 100;
    let mut last_states = (0..states_to_keep)
        .map(|_| platform.clone())
        .collect::<Vec<_>>();
    let mut state_idx = 0;

    //println!("{:?}", platform);
    for i in 0..cycle_count {
        platform.cycle();

        // check if we went back to a previous state
        let exists_at_idx = last_states.iter().position(|s| s.tiles.eq(&platform.tiles));

        if let Some(exists_at_idx) = exists_at_idx {
            println!("Found previous state at idx {}", exists_at_idx);
            let cycle_length = i - exists_at_idx;
            println!("Cycle length is {}", cycle_length);

            // simulate all those loops (skip)
            let remaining_cycles = (cycle_count - i) / cycle_length;

            // and finish off the remaining tasks
            let to_finish_off = (cycle_count - i - (remaining_cycles * cycle_length)) - 1;
            println!("To finish off: {}", to_finish_off);
            for _ in 0..to_finish_off {
                platform.cycle();
            }

            return Some(platform.total_load_north_beam());
        }

        last_states[state_idx] = platform.clone();
        state_idx = (state_idx + 1) % states_to_keep; // there is probably a ring buffer lib somewhere
    }

    // we'll never reach that point
    Some(platform.total_load_north_beam())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(64));
    }
    #[test]
    fn test_cycle() {
        let initial = &advent_of_code::template::read_file("examples", DAY, 1);
        let expected_after_1 = &advent_of_code::template::read_file("examples", DAY, 2);
        let expected_after_2 = &advent_of_code::template::read_file("examples", DAY, 3);
        let expected_after_3 = &advent_of_code::template::read_file("examples", DAY, 4);
        let mut platform = Platform::from(initial.as_str());

        platform.cycle();
        assert!(platform
            .tiles
            .eq(&Platform::from(expected_after_1.as_str()).tiles),);
        platform.cycle();
        assert_eq!(
            platform.tiles,
            Platform::from(expected_after_2.as_str()).tiles
        );
        platform.cycle();
        assert_eq!(
            platform.tiles,
            Platform::from(expected_after_3.as_str()).tiles
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(136));
    }
}
