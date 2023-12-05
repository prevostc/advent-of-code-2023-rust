use std::cmp;
use std::ops::Range;

advent_of_code::solution!(5);

type Mappings = [Vec<[Range<u64>; 2]>];

fn parse_mapping_ranges(input: &str) -> Vec<Vec<[Range<u64>; 2]>> {
    // FOR THIS TO WORK ADD A NEWLINE AT THE END OF THE INPUT
    let mut all_mappings: Vec<Vec<[Range<u64>; 2]>> = vec![];
    let mut mapping: Vec<[Range<u64>; 2]> = vec![];
    for l in input.lines().skip(2) {
        if l.contains("-to-") {
            continue;
        }
        // start of a new block
        if l.len() <= 0 {
            if mapping.len() > 0 {
                // this is sorting by source
                mapping.sort_by(|a, b| a[0].start.cmp(&b[0].start));

                // fix holes in mapping to make searching simpler
                mapping.push([u64::MAX..u64::MAX, u64::MAX..u64::MAX]);
                let mut full_mapping: Vec<[Range<u64>; 2]> = vec![];
                let mut cur_value = 0;
                let mut map_idx = 0;
                while cur_value < u64::MAX {
                    if cur_value < mapping[map_idx][0].start {
                        full_mapping.push([
                            cur_value..mapping[map_idx][0].start,
                            cur_value..mapping[map_idx][0].start,
                        ]);
                    }
                    let range = mapping[map_idx].clone();
                    cur_value = range[0].end;
                    full_mapping.push(range);
                    map_idx += 1;
                }
                full_mapping.pop();

                all_mappings.push(full_mapping);
            }
            mapping = vec![];
            continue;
        }

        let mut nums: [u64; 3] = [0; 3];
        for (idx, n) in l.split(" ").enumerate() {
            nums[idx] = n.parse::<u64>().unwrap();
        }
        // reverse this src/dst nonsense
        (nums[0], nums[1]) = (nums[1], nums[0]);

        mapping.push([nums[0]..(nums[0] + nums[2]), nums[1]..(nums[1] + nums[2])]);
    }
    all_mappings
}

fn solve_lowest_location_range(all_mappings: &Mappings, elem_range: Range<u64>) -> u64 {
    if elem_range.is_empty() {
        return u64::MAX;
    }
    if all_mappings.len() <= 0 {
        return u64::MAX;
    }

    let mapping = all_mappings[0].clone();
    let map = mapping
        .into_iter()
        .find(|map| map[0].contains(&elem_range.start))
        .unwrap();

    // the range always intersect
    let range_in_map = elem_range.start..cmp::min(map[0].end, elem_range.end);
    let range_after_map = range_in_map.end..elem_range.end;
    let mapped_range;

    // some unsigned int shenanigans
    if map[0].start <= map[1].start {
        let shift_right = map[1].start - map[0].start;
        mapped_range = (range_in_map.start + shift_right)..(range_in_map.end + shift_right);
    } else {
        let shift_left = map[0].start - map[1].start;
        mapped_range = (range_in_map.start - shift_left)..(range_in_map.end - shift_left);
    }

    if all_mappings.len() == 1 {
        let res = cmp::min(mapped_range.start, range_after_map.start);
        return res;
    }
    let next_mappings = &all_mappings[1..];
    return cmp::min(
        solve_lowest_location_range(next_mappings, mapped_range),
        solve_lowest_location_range(&all_mappings, range_after_map),
    );
}

pub fn part_one(input: &str) -> Option<u32> {
    let all_mappings = parse_mapping_ranges(input);

    let seeds: Vec<_> = input.lines().nth(0).unwrap()["seeds: ".len()..]
        .split(" ")
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    let lowest_loc = seeds
        .into_iter()
        .map(|seed| solve_lowest_location_range(&all_mappings, seed..(seed + 1)))
        .min()
        .unwrap();

    Some(lowest_loc.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let all_mappings = parse_mapping_ranges(input);

    let seed_ranges = input.lines().nth(0).unwrap()["seeds: ".len()..]
        .split(" ")
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|seed_range| seed_range[0]..(seed_range[0] + seed_range[1]))
        .collect::<Vec<_>>();

    let lowest_loc = seed_ranges
        .into_iter()
        .map(|seed_range| solve_lowest_location_range(&all_mappings, seed_range))
        .min()
        .unwrap();

    Some(lowest_loc.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(46));
    }
}
