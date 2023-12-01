advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let mut nums = line.chars().filter(|c| c.is_numeric());
            let c1 = nums.nth(0).unwrap();
            let c2 = nums.nth_back(0).unwrap_or(c1);
            let n1 = c1.to_digit(10).unwrap();
            let n2 = c2.to_digit(10).unwrap();
            n1 * 10 + n2
        })
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let num_strs = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let nums = num_strs
        .iter()
        .enumerate()
        .map(|(v, s)| ((v + 1) as u32, (*s).to_owned()))
        .collect::<Vec<_>>();
    let nums_rev = nums
        .iter()
        .map(|(i, s)| (*i, s.chars().rev().collect::<String>().to_owned()))
        .collect::<Vec<_>>();

    fn get_n(s: &str, ns: Vec<(u32, String)>) -> Option<u32> {
        for (i, c) in s.char_indices() {
            if let Some(d) = c.to_digit(10) {
                return Some(d);
            }
            for (v, num) in ns.iter() {
                if s[i..].starts_with(num) {
                    return Some(*v);
                }
            }
        }

        None
    }

    let n = input
        .lines()
        .map(|line| {
            let n1 = get_n(line, nums.clone()).unwrap();
            let n2 = get_n(
                line.chars().rev().collect::<String>().as_str(),
                nums_rev.clone(),
            )
            .unwrap();
            n1 * 10 + n2
        })
        .sum::<u32>();

    Some(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}
