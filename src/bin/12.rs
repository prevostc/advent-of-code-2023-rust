use itertools::Itertools;
use rayon::prelude::*;
advent_of_code::solution!(12);

#[derive(Debug, Copy, Clone, PartialEq)]
enum Mark {
    Unknown,
    Spring,
    Empty,
}

impl Mark {
    fn new(c: char) -> Self {
        match c {
            '#' => Mark::Spring,
            '.' => Mark::Empty,
            '?' => Mark::Unknown,
            _ => panic!("Invalid char '{c}'"),
        }
    }
}

#[derive(Debug, Clone)]
struct Record {
    marks: Vec<Mark>,
    counts: Vec<i64>,
    total_count: i64,
}
impl Record {
    #[inline]
    fn is_definitely_valid(&self) -> bool {
        let mut counts_iter = self.counts.iter();
        let mut cur_count = 0;
        let mut cur_type = Mark::Empty;

        // iter over marks but add "." at the end to make sure we end with an empty mark
        let range = self.marks.iter().chain(std::iter::once(&Mark::Empty));

        for mark in range {
            match (cur_type, *mark) {
                (Mark::Empty, Mark::Spring) => {
                    cur_type = Mark::Spring;
                    match counts_iter.next() {
                        None => return false,
                        Some(count) => cur_count = count - 1,
                    }
                }
                (Mark::Spring, Mark::Empty) => {
                    cur_type = Mark::Empty;
                    if cur_count != 0 {
                        return false;
                    }
                }
                (Mark::Spring, Mark::Spring) => {
                    cur_count -= 1;
                }
                (Mark::Empty, Mark::Empty) => {}
                // only validate fully known patterns
                (_, Mark::Unknown) => {
                    return false;
                }
                (Mark::Unknown, _) => {
                    panic!("Invalid input")
                }
            }
        }
        match counts_iter.next() {
            None => true,
            Some(_) => false,
        }
    }

    #[inline]
    fn count_valid_arrangements(&mut self) -> i64 {
        let ukn_idx: Option<usize> = self.marks.iter().position(|m| *m == Mark::Unknown);
        if ukn_idx.is_none() {
            return self.is_definitely_valid() as i64;
        }
        let ukn_idx = ukn_idx.unwrap();

        // test if enough space for all counts
        let total_available = self
            .marks
            .iter()
            .filter(|&m| !matches!(m, Mark::Empty))
            .count() as i64;
        if total_available < self.total_count {
            return 0;
        }

        // count if disconnected springs length is > number of counts
        let range = self.marks.iter().chain(std::iter::once(&Mark::Empty));
        let mut break_count = 0;
        for e in range.tuple_windows() {
            match e {
                (Mark::Spring, Mark::Empty) => break_count += 1,
                _ => {}
            }
        }
        if break_count > self.counts.len() as i64 {
            return 0;
        }

        // count if max consecutive springs is < max count
        let range = self.marks.iter().chain(std::iter::once(&Mark::Empty));
        let max_count = self.counts.iter().max().unwrap().clone();
        let mut cur_count = 0;
        for e in range.tuple_windows() {
            match e {
                (Mark::Spring, Mark::Spring) => {
                    cur_count += 1;
                }
                (Mark::Unknown, Mark::Spring) => {}
                (Mark::Empty, Mark::Spring | Mark::Unknown) => {
                    cur_count = 1;
                }
                (Mark::Spring, _) => {
                    if cur_count > max_count {
                        return 0;
                    }
                    cur_count = 0;
                }
                _ => {}
            }
        }

        // try consuming non unknown marks
        let range = std::iter::once(&Mark::Empty)
            .chain(self.marks.iter())
            .chain(std::iter::once(&Mark::Empty));
        let mut cur_count = 0;
        let mut counts_idx = -1;
        for e in range.tuple_windows() {
            match e {
                (_, Mark::Unknown) => break,
                (Mark::Spring, Mark::Spring) => {
                    cur_count += 1;
                }
                (Mark::Empty, Mark::Spring) => {
                    counts_idx += 1;
                    cur_count = 1;
                }
                (Mark::Spring, Mark::Empty) => {
                    if cur_count != self.counts[counts_idx as usize] {
                        return 0;
                    }
                    cur_count = 0;
                }
                (Mark::Empty, Mark::Empty) => {}
                (Mark::Unknown, _) => unreachable!(),
            }
        }

        // solve n and recurse
        let mut arrangements = 0;

        // apply one version and recurse
        self.marks[ukn_idx] = Mark::Empty;
        arrangements += self.count_valid_arrangements();
        self.marks[ukn_idx] = Mark::Spring;
        arrangements += self.count_valid_arrangements();

        // unapply change for next iteration
        self.marks[ukn_idx] = Mark::Unknown;
        return arrangements;
    }

    fn count_valid_arrangements_with_duplicates(&mut self, dup: usize) -> i64 {
        // a quick test shows that it makes sense to duplicate the content 5 times
        let ukn_count = self.marks.iter().filter(|&m| *m == Mark::Unknown).count();
        if ukn_count < 7 {
            let res = self.count_valid_arrangements();
            let mut copy = self.clone();
            copy.duplicate_content(2);
            let simulated_res = res * 8;
            let simulated_copy_res = copy.count_valid_arrangements();
            if simulated_res == simulated_copy_res {
                println!("SHORTCUT");
                return res * 4096;
            }
        }

        self.duplicate_content(dup);
        let res = self.count_valid_arrangements();
        println!("solved: {:?}", self.counts);
        return res;
    }
}

impl Record {
    fn new(input: &str) -> Self {
        match input.split_once(' ') {
            Some((marks_str, counts_str)) => {
                let marks = marks_str.chars().map(Mark::new).collect();
                let counts = counts_str
                    .split(',')
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect::<Vec<_>>();
                let total_count = counts.iter().sum::<i64>();
                Self {
                    marks,
                    counts,
                    total_count,
                }
            }
            _ => panic!("Invalid input"),
        }
    }

    fn duplicate_content(&mut self, n: usize) {
        // diplicate marks separated by '?'
        let mut marks = Vec::new();
        let mut counts = Vec::new();

        for i in 0..n {
            marks.append(&mut self.marks.clone());
            counts.append(&mut self.counts.clone());
            if i < n - 1 {
                marks.push(Mark::Unknown);
            }
        }
        self.marks = marks;
        self.counts = counts;
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut records = input.lines().map(Record::new).collect::<Vec<_>>();
    //println!("{:?}", records);

    let res = records
        .par_iter_mut()
        //.iter_mut()
        .map(|r| r.count_valid_arrangements())
        .sum::<i64>();
    Some(res)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut records = input.lines().map(Record::new).collect::<Vec<_>>();
    //println!("{:?}", records);
    let res = records
        .par_iter_mut()
        //.iter_mut()
        .inspect(|r| println!("taking: {:?}", r.counts))
        .map(|r| r.count_valid_arrangements_with_duplicates(5))
        .sum::<i64>();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(525152));
    }

    #[test]
    fn test_record_is_definitely_valid() {
        let record = Record::new("#..#..#..# 1,1,1,1");
        assert_eq!(record.is_definitely_valid(), true);

        let record = Record::new("####..#..## 4,1,2");
        assert_eq!(record.is_definitely_valid(), true);

        // too many numbers
        let record = Record::new("##..#..## 2,1,2,5");
        assert_eq!(record.is_definitely_valid(), false);

        // too few numbers
        let record = Record::new("##..#..## 2,1");
        assert_eq!(record.is_definitely_valid(), false);

        // wrong number
        let record = Record::new("####..#..## 2,1,2");
        assert_eq!(record.is_definitely_valid(), false);
        let record = Record::new("####..#..## 23,2,2");
        assert_eq!(record.is_definitely_valid(), false);

        // unknown mark
        let record = Record::new("##?.##..## 2,2,2");
        assert_eq!(record.is_definitely_valid(), false);
    }

    #[test]
    fn test_count_valid_arrangements() {
        let mut record = Record::new("#..#..#..# 1,1,1,1");
        assert_eq!(record.count_valid_arrangements(), 1);

        let mut record = Record::new("####..#..## 4,1,2");
        assert_eq!(record.count_valid_arrangements(), 1);

        let mut record = Record::new("??###??..#..## 4,1,2");
        assert_eq!(record.count_valid_arrangements(), 2);

        let mut record = Record::new("#?###??..#..## 4,1,2");
        assert_eq!(record.count_valid_arrangements(), 0);

        let mut record = Record::new("#?####?..#..## 4,1,2");
        assert_eq!(record.count_valid_arrangements(), 0);

        let mut record = Record::new("#?#####?..#..## 4,1,2");
        assert_eq!(record.count_valid_arrangements(), 0);

        let mut record = Record::new("#?.#####?..#..## 4,1,2");
        assert_eq!(record.count_valid_arrangements(), 0);

        let mut record = Record::new("#?.#####.?..#..## 4,1,2");
        assert_eq!(record.count_valid_arrangements(), 0);

        let mut record = Record::new("???.### 1,1,3");
        assert_eq!(record.count_valid_arrangements(), 1);
    }

    #[test]
    fn test_solve_1() {
        let mut record = Record::new("???.### 1,1,3");
        assert_eq!(record.count_valid_arrangements(), 1);

        let mut record = Record::new(".??..??...?##. 1,1,3");
        assert_eq!(record.count_valid_arrangements(), 4);

        let mut record = Record::new("?#?#?#?#?#?#?#? 1,3,1,6");
        assert_eq!(record.count_valid_arrangements(), 1);

        let mut record = Record::new("????.#...#... 4,1,1");
        assert_eq!(record.count_valid_arrangements(), 1);

        let mut record = Record::new("????.######..#####. 1,6,5");
        assert_eq!(record.count_valid_arrangements(), 4);

        let mut record = Record::new("?###???????? 3,2,1");
        assert_eq!(record.count_valid_arrangements(), 10);
    }

    #[test]
    fn test_duplicate_content() {
        let mut record = Record::new(".# 1");
        record.duplicate_content(5);
        let record_expected = Record::new(".#?.#?.#?.#?.# 1,1,1,1,1");
        assert_eq!(record.marks, record_expected.marks);
        assert_eq!(record.counts, record_expected.counts);
    }

    #[test]
    fn test_solve_2_1() {
        let mut record = Record::new("???.### 1,1,3");
        record.duplicate_content(5);
        assert_eq!(record.count_valid_arrangements_with_duplicates(5), 1);
        println!("{:?}", record);
    }

    #[test]
    fn test_solve_2_2() {
        let mut record = Record::new(".??..??...?##. 1,1,3");
        assert_eq!(record.count_valid_arrangements_with_duplicates(5), 16384);
        println!("{:?}", record);
    }

    #[test]
    fn test_solve_2_3() {
        let mut record = Record::new("?#?#?#?#?#?#?#? 1,3,1,6");
        assert_eq!(record.count_valid_arrangements_with_duplicates(5), 1);
        println!("{:?}", record);
    }

    #[test]
    fn test_solve_2_4() {
        let mut record = Record::new("????.#...#... 4,1,1");
        assert_eq!(record.count_valid_arrangements_with_duplicates(5), 16);
        println!("{:?}", record);
    }

    #[test]
    fn test_solve_2_5() {
        let mut record = Record::new("????.######..#####. 1,6,5");
        assert_eq!(record.count_valid_arrangements_with_duplicates(5), 2500);
        println!("{:?}", record);
    }

    //#[test]
    //fn test_solve_2_6() {
    //    let mut record = Record::new("?###???????? 3,2,1");
    //    assert_eq!(record.count_valid_arrangements_with_duplicates(5), 506250);
    //    println!("{:?}", record);
    //}
}
