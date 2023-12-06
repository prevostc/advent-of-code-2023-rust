advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let data = input
        .lines()
        .map(|l| &l["Time:     ".len()..])
        .map(|l| {
            l.split(|c: char| c.is_ascii_whitespace())
                .filter(|p| !p.is_empty())
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let races = data[0]
        .iter()
        .enumerate()
        .map(|(i, t)| (t, data[1][i]))
        .collect::<Vec<_>>();

    let mut res = 1;
    for (race_time, race_distance) in races {
        let mut valid_ways_count = 0;
        for time_pressing in 1..(race_time - 1) {
            let distance = time_pressing * (race_time - time_pressing);
            if distance > race_distance {
                valid_ways_count += 1;
            }
        }
        res *= valid_ways_count;
    }

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    // I entered "find x roots of y = x * (time - x) - distance" into wolfram alpha
    // and subtracted the lowest root to the highest root
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, None);
    }
}
