use bigdecimal::BigDecimal;
use itertools::Itertools;
use std::str::FromStr;

advent_of_code::solution!(24);

#[derive(Debug, Clone)]
struct Hail {
    origin: [BigDecimal; 3],
    direction: [BigDecimal; 3],
}

impl Hail {
    fn new(origin: [BigDecimal; 3], direction: [BigDecimal; 3]) -> Self {
        Self { origin, direction }
    }

    fn parse_line(input: &str) -> Self {
        let (position, velocity) = input.split_once("@").unwrap();
        let oxyz = position
            .split(", ")
            .map(str::trim)
            .map(BigDecimal::from_str)
            .map(Result::unwrap)
            .collect::<Vec<_>>();
        let dxyz = velocity
            .split(", ")
            .map(str::trim)
            .map(BigDecimal::from_str)
            .map(Result::unwrap)
            .collect::<Vec<_>>();

        Hail::new(
            [oxyz[0].clone(), oxyz[1].clone(), oxyz[2].clone()],
            [dxyz[0].clone(), dxyz[1].clone(), dxyz[2].clone()],
        )
    }

    fn yx_path_will_intersect_at(&self, other: &Self) -> Option<[BigDecimal; 2]> {
        let ref ax = self.origin[0].clone();
        let ref ay = self.origin[1].clone();
        let ref bx = self.origin[0].clone() + self.direction[0].clone();
        let ref by = self.origin[1].clone() + self.direction[1].clone();
        let ref cx = other.origin[0].clone();
        let ref cy = other.origin[1].clone();
        let ref dx = other.origin[0].clone() + other.direction[0].clone();
        let ref dy = other.origin[1].clone() + other.direction[1].clone();

        // first, find if the lines are parallel
        let ref denom = (bx - ax) * (dy - cy) - (by - ay) * (dx - cx);
        if *denom == BigDecimal::from(0) {
            return None;
        }
        // then, find the intersection point of the lines represented by lines ab and cd
        let ref intersection_x = -((bx - ax) * (dx - cx) * (cy - ay) + (by - ay) * (dx - cx) * ax
            - (dy - cy) * (bx - ax) * cx)
            / denom;
        let ref intersection_y = ((by - ay) * (dy - cy) * (cx - ax) + (bx - ax) * (dy - cy) * ay
            - (dx - cx) * (by - ay) * cy)
            / denom;

        // check if the intersection is in the right direction for both rays
        let dir_1 = (intersection_x - ax) / &self.direction[0];
        let dir_2 = (intersection_x - cx) / &other.direction[0];

        if dir_1 >= BigDecimal::from(0) && dir_2 >= BigDecimal::from(0) {
            Some([intersection_x.clone(), intersection_y.clone()])
        } else {
            None
        }
    }
}

fn solve_part1(input: &str, test_area: [BigDecimal; 2]) -> i32 {
    let hail_paths: Vec<Hail> = input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(Hail::parse_line)
        .collect::<Vec<_>>();

    let mut intersecting_in_test_area = 0;
    for ab in hail_paths.iter().combinations(2) {
        let a = ab[0];
        let b = ab[1];

        if let Some(intersection) = a.yx_path_will_intersect_at(&b) {
            if intersection[0] >= test_area[0]
                && intersection[0] <= test_area[1]
                && intersection[1] >= test_area[0]
                && intersection[1] <= test_area[1]
            {
                intersecting_in_test_area += 1;
            }
        }
    }
    intersecting_in_test_area
}

pub fn part_one(input: &str) -> Option<u32> {
    let test_area: [BigDecimal; 2] = [
        BigDecimal::from_str("200000000000000").unwrap(),
        BigDecimal::from_str("400000000000000").unwrap(),
    ];
    let result = solve_part1(input, test_area);
    Some(result as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part1(
            &advent_of_code::template::read_file("examples", DAY, 1),
            [BigDecimal::from(7), BigDecimal::from(27)],
        );
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, None);
    }

    #[test]
    fn test_example_intersect_1() {
        let ray1 = Hail::parse_line("19, 13, 30 @ -2, 1, -2");
        let ray2 = Hail::parse_line("18, 19, 22 @ -1, -1, -2");
        let result = ray1.yx_path_will_intersect_at(&ray2);
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(
            result[0].with_prec(10),
            BigDecimal::from_str("14.33333333").unwrap()
        );
        assert_eq!(
            result[1].with_prec(10),
            BigDecimal::from_str("15.33333333").unwrap()
        );
    }

    #[test]
    fn test_example_intersect_2() {
        let ray1 = Hail::parse_line("19, 13, 30 @ -2, 1, -2");
        let ray2 = Hail::parse_line("20, 25, 34 @ -2, -2, -4");
        let result = ray2.yx_path_will_intersect_at(&ray1);
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(
            result[0].with_prec(10),
            BigDecimal::from_str("11.66666667").unwrap()
        );
        assert_eq!(
            result[1].with_prec(10),
            BigDecimal::from_str("16.66666667").unwrap()
        );
    }

    #[test]
    fn test_example_intersect_3() {
        let ray1 = Hail::parse_line("19, 13, 30 @ -2, 1, -2");
        let ray2 = Hail::parse_line("12, 31, 28 @ -1, -2, -1");
        let result = ray2.yx_path_will_intersect_at(&ray1);
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result[0], BigDecimal::from_str("6.2").unwrap());
        assert_eq!(result[1], BigDecimal::from_str("19.4").unwrap());
    }

    #[test]
    fn test_example_intersect_past() {
        let ray1 = Hail::parse_line("19, 13, 30 @ -2, 1, -2");
        let ray2 = Hail::parse_line("20, 19, 15 @ 1, -5, -3");
        let result = ray1.yx_path_will_intersect_at(&ray2);
        assert_eq!(result, None);
    }

    #[test]
    fn test_line_intersects_at_parallel() {
        let ray1 = Hail::parse_line("18, 19, 22 @ -1, -1, -2");
        let ray2 = Hail::parse_line("20, 25, 34 @ -2, -2, -4");
        let result = ray1.yx_path_will_intersect_at(&ray2);
        assert_eq!(result, None);
    }
}
