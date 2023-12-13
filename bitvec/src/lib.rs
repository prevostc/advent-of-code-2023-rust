use std::fmt::{Display, Formatter, Result};
use std::iter::FromIterator;
use std::ops::Index;
use std::ops::{BitAnd, BitOr, BitXor};

#[derive(Debug, Clone, Copy)]
pub struct BitVec64 {
    content: u64,
    size: u8,
}

impl Index<usize> for BitVec64 {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.size as usize);
        // index by highest significant bit first
        let index = self.size as usize - index - 1;
        let mask = 1 << index;
        if self.content & mask == 0 {
            &false
        } else {
            &true
        }
    }
}

impl PartialEq for BitVec64 {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.content == other.content
    }
}

impl BitXor for BitVec64 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        assert_eq!(self.size, rhs.size);
        Self {
            content: self.content ^ rhs.content,
            size: self.size,
        }
    }
}

impl BitOr for BitVec64 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        assert_eq!(self.size, rhs.size);
        Self {
            content: self.content | rhs.content,
            size: self.size,
        }
    }
}

impl BitAnd for BitVec64 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        assert_eq!(self.size, rhs.size);
        Self {
            content: self.content & rhs.content,
            size: self.size,
        }
    }
}

impl BitVec64 {
    pub fn from_content(content: u64, size: u8) -> Self {
        assert!(size <= 64);
        Self { content, size }
    }

    pub fn from_size(size: u8) -> Self {
        assert!(size <= 64);
        Self { content: 0, size }
    }

    pub fn from_str(input: &str, true_value: char, false_value: char) -> Self {
        let len = input.len();
        assert!(len <= 64);

        let content = input.chars().fold(0b0, |acc, c| {
            let bit = match c {
                c if c == true_value => 1,
                c if c == false_value => 0,
                _ => panic!("invalid char"),
            };
            (acc << 1) | bit
        });

        Self {
            content,
            size: len as u8,
        }
    }

    pub fn slice(&self, start: usize, end: usize) -> u64 {
        assert!(start < end);
        assert!(end <= self.size as usize);
        // we want the bits from most significant to least significant exclusively [start, end)
        let start = self.size as usize - start - 1;
        let end = self.size as usize - end;
        let mask = (1 << (start + 1)) - (1 << end);
        (self.content & mask) >> end
    }

    pub fn len(&self) -> usize {
        self.size as usize
    }

    pub fn count_ones(&self) -> u32 {
        self.content.count_ones()
    }

    pub fn count_zeros(&self) -> u32 {
        (self.size as u32) - self.content.count_ones()
    }
}

pub struct BitVec64Fmt<'a> {
    bitvec: &'a BitVec64,
    true_str: char,
    false_str: char,
}

impl BitVec64 {
    pub fn as_fmt(&self, true_str: char, false_str: char) -> BitVec64Fmt {
        BitVec64Fmt {
            bitvec: self,
            true_str,
            false_str,
        }
    }
}

impl Display for BitVec64Fmt<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for i in 0..self.bitvec.len() {
            if self.bitvec[i] {
                write!(f, "{}", self.true_str)?;
            } else {
                write!(f, "{}", self.false_str)?;
            }
        }
        Ok(())
    }
}

impl Display for BitVec64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.as_fmt('1', '0').fmt(f)
    }
}

pub struct BitVec64Iter {
    bitvec: BitVec64,
    pos: usize,
}

impl Iterator for BitVec64Iter {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.bitvec.len() {
            None
        } else {
            let item = self.bitvec[self.pos];
            self.pos += 1;
            Some(item)
        }
    }
}
impl FromIterator<bool> for BitVec64 {
    fn from_iter<I: IntoIterator<Item = bool>>(iter: I) -> Self {
        let mut content = 0;
        let mut size = 0;
        for bit in iter {
            content = (content << 1) | (bit as u64);
            size += 1;
        }
        Self { content, size }
    }
}

impl BitVec64 {
    pub fn iter(&self) -> BitVec64Iter {
        BitVec64Iter {
            bitvec: self.clone(),
            pos: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitvec64_create() {
        let bv = BitVec64 {
            content: 0b1010,
            size: 4,
        };
        assert_eq!(bv[0], true);
        assert_eq!(bv[1], false);
        assert_eq!(bv[2], true);
        assert_eq!(bv[3], false);
    }

    #[test]
    fn test_bitvec64_fmt() {
        let bv = BitVec64 {
            content: 0b1010,
            size: 4,
        };
        assert_eq!(format!("{}", bv.as_fmt('#', '.')), "#.#.");
    }

    #[test]
    fn test_bitvec64_from_str() {
        let bv = BitVec64::from_str("##..", '#', '.');
        assert_eq!(bv.content, 0b1100);
        assert_eq!(bv.size, 4);
    }

    #[test]
    fn test_bitvec64_from_content() {
        let bv = BitVec64::from_content(0b1100, 4);
        assert_eq!(bv.content, 0b1100);
        assert_eq!(bv.size, 4);
        assert_eq!(bv[0], true);
        assert_eq!(bv[1], true);
        assert_eq!(bv[2], false);
        assert_eq!(bv[3], false);
    }

    #[test]
    fn test_bitvec64_from_size() {
        let bv = BitVec64::from_size(4);
        assert_eq!(bv.content, 0b0);
        assert_eq!(bv.size, 4);
        assert_eq!(bv[0], false);
        assert_eq!(bv[1], false);
        assert_eq!(bv[2], false);
        assert_eq!(bv[3], false);
    }

    #[test]
    fn test_bitvec64_len() {
        let bv = BitVec64::from_content(0b1100, 4);
        assert_eq!(bv.len(), 4);
    }

    #[test]
    fn test_bitvec64_slice() {
        let bv = BitVec64::from_content(0b1100, 4);
        assert_eq!(bv.slice(0, 1), 0b1);
        assert_eq!(bv.slice(0, 2), 0b11);
        assert_eq!(bv.slice(1, 3), 0b10);
        assert_eq!(bv.slice(0, 4), 0b1100);
        assert_eq!(bv.slice(2, 4), 0b0);
        assert_eq!(bv.slice(2, 3), 0b0);
    }

    #[test]
    fn test_bitvec64_equal() {
        let bv1 = BitVec64::from_content(0b1100, 4);
        let bv2 = BitVec64::from_content(0b1100, 4);
        assert_eq!(bv1, bv2);
    }

    #[test]
    fn test_bitvec64_xor() {
        let bv1 = BitVec64::from_content(0b1100, 4);
        let bv2 = BitVec64::from_content(0b1010, 4);
        let bv3 = bv1 ^ bv2;
        assert_eq!(bv3.content, 0b0110);
        assert_eq!(bv3.size, 4);
    }

    #[test]
    fn test_bitvec64_or() {
        let bv1 = BitVec64::from_content(0b1100, 4);
        let bv2 = BitVec64::from_content(0b1010, 4);
        let bv3 = bv1 | bv2;
        assert_eq!(bv3.content, 0b1110);
        assert_eq!(bv3.size, 4);
    }

    #[test]
    fn test_bitvec64_and() {
        let bv1 = BitVec64::from_content(0b1100, 4);
        let bv2 = BitVec64::from_content(0b1010, 4);
        let bv3 = bv1 & bv2;
        assert_eq!(bv3.content, 0b1000);
        assert_eq!(bv3.size, 4);
    }

    #[test]
    fn test_bitvec64_count_ones() {
        let bv = BitVec64::from_content(0b1100, 4);
        assert_eq!(bv.count_ones(), 2);
    }

    #[test]
    fn test_bitvec64_count_zeros() {
        let bv = BitVec64::from_content(0b1100, 4);
        assert_eq!(bv.count_zeros(), 2);
    }

    #[test]
    fn test_bitvec64_iter() {
        let bv = BitVec64::from_content(0b1100, 4);
        let mut iter = bv.iter();
        assert_eq!(iter.next(), Some(true));
        assert_eq!(iter.next(), Some(true));
        assert_eq!(iter.next(), Some(false));
        assert_eq!(iter.next(), Some(false));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_bitvec64_from_iter() {
        let bv = BitVec64::from_iter(vec![true, true, false, false]);
        assert_eq!(bv.content, 0b1100);
        assert_eq!(bv.size, 4);
    }
}
