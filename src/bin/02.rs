use std::cmp;
advent_of_code::solution!(2);

struct Bag {
    r: u32,
    g: u32,
    b: u32,
}

impl Bag {
    fn fits_into(&self, containing: &Bag) -> bool {
        return self.r <= containing.r && self.g <= containing.g && self.b <= containing.b;
    }

    fn grow_to_fit(&mut self, to_be_contained: &Bag) {
        self.r = cmp::max(self.r, to_be_contained.r);
        self.g = cmp::max(self.g, to_be_contained.g);
        self.b = cmp::max(self.b, to_be_contained.b);
    }

    fn parse_draw(draw: &str) -> Bag {
        let mut bag = Bag { r: 0, g: 0, b: 0 };
        for r in draw.split(",") {
            let n = r
                .trim_start()
                .split(' ')
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            if r.ends_with("red") {
                bag.r += n;
            } else if r.ends_with("green") {
                bag.g += n;
            } else if r.ends_with("blue") {
                bag.b += n;
            }
        }
        bag
    }

    fn power(self) -> u32 {
        self.r * self.g * self.b
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let max = Bag {
        r: 12,
        g: 13,
        b: 14,
    };

    // let's not use regex just yet, need to practice raw rust
    let res = input
        .lines()
        .enumerate()
        .map(|(idx, l)| {
            for draw in l.split(": ").last().unwrap().split("; ") {
                let bag = Bag::parse_draw(draw);
                if !bag.fits_into(&max) {
                    return 0;
                }
            }
            u32::try_from(idx).unwrap() + 1
        })
        .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let res = input
        .lines()
        .map(|l| {
            let mut min_bag = Bag { r: 0, g: 0, b: 0 };
            l.split(": ")
                .last()
                .unwrap()
                .split("; ")
                .map(Bag::parse_draw)
                .for_each(|bag| min_bag.grow_to_fit(&bag));
            min_bag.power()
        })
        .sum();

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(2286));
    }
}
