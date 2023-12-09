use grid::*;

advent_of_code::solution!(9);

struct SensorData {
    pyramid: Grid<Option<i64>>,
}

impl SensorData {
    fn from_str(line: &str, p1: bool) -> Self {
        let c = line.split(" ").count() + 1;
        let h = c - 1;

        let mut pyramid: Grid<Option<i64>> = Grid::new(h, c);
        let s = if p1 {
            line.to_owned() + " x"
        } else {
            "x ".to_owned() + line
        };
        let values = s
            .split(" ")
            .map(|cs| cs.parse::<i64>().ok())
            .collect::<Vec<_>>();
        pyramid.insert_row(0, values);

        return Self { pyramid };
    }
    fn predict_p1(&mut self) -> i64 {
        let h = self.pyramid.rows();
        let w = self.pyramid.cols();

        for l in 1..h {
            let mut all_zero = true;
            for c in 0..(w - l) {
                let a = self.pyramid[(l - 1, c)];
                let b = self.pyramid[(l - 1, c + 1)];

                match (a, b) {
                    (Some(va), Some(vb)) => {
                        let res = vb - va;
                        if res != 0 {
                            all_zero = false;
                        }
                        self.pyramid[(l, c)] = Some(res);
                    }
                    _ => {}
                }
            }
            if all_zero {
                self.pyramid[(l, w - l - 1)] = Some(0);

                // fill back up
                for nl in (0..l).rev() {
                    let c = w - nl - 1;

                    let a = self.pyramid[(nl + 1, c - 1)];
                    let b = self.pyramid[(nl, c - 1)];

                    match (a, b) {
                        (Some(va), Some(vb)) => {
                            self.pyramid[(nl, c)] = Some(vb + va);
                        }
                        _ => panic!("Nope"),
                    }
                }

                return self.pyramid[(0, w - 1)].unwrap();
            }
        }
        panic!("Could not predict")
    }

    fn predict_p2(&mut self) -> i64 {
        let h = self.pyramid.rows();
        let w = self.pyramid.cols();

        for l in 1..h {
            let mut all_zero = true;
            for c in 1..(w - l) {
                let a = self.pyramid[(l - 1, c)];
                let b = self.pyramid[(l - 1, c + 1)];

                match (a, b) {
                    (Some(va), Some(vb)) => {
                        let res = vb - va;
                        if res != 0 {
                            all_zero = false;
                        }
                        self.pyramid[(l, c)] = Some(res);
                    }
                    _ => {}
                }
            }
            if all_zero {
                self.pyramid[(l, 0)] = Some(0);

                // fill back up
                for nl in (0..l).rev() {
                    let c = 0;

                    let a = self.pyramid[(nl + 1, c)];
                    let b = self.pyramid[(nl, c + 1)];

                    match (a, b) {
                        (Some(va), Some(vb)) => {
                            self.pyramid[(nl, c)] = Some(vb - va);
                        }
                        _ => panic!("Nope"),
                    }
                }

                return self.pyramid[(0, 0)].unwrap();
            }
        }
        panic!("Could not predict")
    }

    fn print(&self) {
        let h = self.pyramid.rows();
        let w = self.pyramid.cols();
        let space = "    ";
        for l in 0..h {
            for _ in 0..l {
                print!("{space}");
            }
            for c in 0..(w - l) {
                match self.pyramid[(l, c)] {
                    None => print!("{}{space}{space}", "x"),
                    Some(v) => print!("{}{space}{space}", v),
                }
            }
            println!("");
        }
        println!("");
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let data = input
        .lines()
        .map(|l| SensorData::from_str(l, true))
        .collect::<Vec<_>>();

    let mut sum = 0;
    for mut d in data {
        sum += d.predict_p1();
        d.print();
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<i64> {
    let data = input
        .lines()
        .map(|l| SensorData::from_str(l, false))
        .collect::<Vec<_>>();

    let mut sum = 0;
    for mut d in data {
        d.print();
        sum += d.predict_p2();
        d.print();
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(2));
    }
}
