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
    let data = input
        .lines()
        .map(|l| &l["Time:     ".len()..])
        .map(|l| l.replace(" ", ""))
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let race_time = data[0];
    let race_distance = data[1];

    let mut valid_ways_count = 0;
    for time_pressing in 1..(race_time - 1) {
        let distance = time_pressing * (race_time - time_pressing);
        if distance >= race_distance {
            valid_ways_count += 1;
        } else if valid_ways_count > 0 {
            break;
        }
    }

    Some(valid_ways_count)
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
        assert_eq!(result, Some(71503));
    }
}
