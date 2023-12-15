use std::collections::VecDeque;
advent_of_code::solution!(15);

fn hash(input: &str) -> u8 {
    input
        .chars()
        .fold(0, |h, c| ((h + c as i32) * 17) & 0xff)
        .try_into()
        .unwrap()
}

pub fn part_one(input: &str) -> Option<i32> {
    let r = input.split(",").map(hash).map(|h| h as i32).sum();
    Some(r)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut boxes: Vec<VecDeque<(&str, u16)>> = vec![VecDeque::new(); 256];
    for action in input.split(",") {
        if action.chars().last().unwrap() == '-' {
            // remove
            let label = action.trim_end_matches('-');
            let label_hash = hash(label);
            let boxe = &mut boxes[label_hash as usize];
            let idx_to_remove = boxe.iter().position(|(l, _)| l.eq(&label));
            if let Some(index) = idx_to_remove {
                boxe.remove(index);
            }
        } else {
            // add
            let (label, focale_len) = action.split_once("=").unwrap();
            let label_hash = hash(label);
            let focale_len = focale_len.parse::<u16>().unwrap();
            let boxe = &mut boxes[label_hash as usize];
            let idx_to_remove = boxe.iter().position(|(l, _)| l.eq(&label));
            if let Some(index) = idx_to_remove {
                boxe[index] = (label, focale_len);
            } else {
                boxe.push_back((label, focale_len));
            }
        }
    }

    let res = boxes
        .iter()
        .enumerate()
        .map(|(box_num, v)| {
            v.iter()
                .enumerate()
                .map(|(slot, (_, focale))| {
                    (box_num as i32 + 1) * (slot as i32 + 1) * (*focale as i32)
                })
                .sum::<i32>()
        })
        .sum();

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(145));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_hash_hash() {
        assert_eq!(hash(""), 0);
        assert_eq!(hash("H"), 200);
        assert_eq!(hash("HA"), 153);
        assert_eq!(hash("HAS"), 172);
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn test_rn_pc() {
        assert_eq!(hash("rn"), 0);
        assert_eq!(hash("pc"), 3);
    }
}
