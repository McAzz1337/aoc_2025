use std::ops::Range;

use crate::read_file_to_vec;

fn find_invalid_ids_p1(ranges: Vec<Range<usize>>) -> usize {
    ranges
        .iter()
        .map(|r| {
            let invalid = (r.start..=r.end)
                .filter(|i| {
                    let s = i.to_string();
                    if s.len() & 0b1 == 0 {
                        let mid = s.len() / 2;
                        let a = s
                            .chars()
                            .take(mid)
                            .collect::<String>()
                            .parse::<usize>()
                            .unwrap();
                        let b = s
                            .chars()
                            .skip(mid)
                            .collect::<String>()
                            .parse::<usize>()
                            .unwrap();
                        s.chars().nth(0).unwrap() != '0' && a == b
                    } else {
                        false
                    }
                })
                .collect::<Vec<_>>();
            invalid
        })
        .flatten()
        .sum()
}

pub fn part1() {
    let file: Vec<Range<usize>> = read_file_to_vec("d2p1", ",")
        .iter()
        .map(|s| s.trim())
        .map(|s| {
            let (start, end) = s.split_once("-").unwrap();
            let start: usize = start.parse().unwrap();
            let end: usize = end.parse().unwrap();
            Range { start, end }
        })
        .collect();
    let res = find_invalid_ids_p1(file);
    println!("d2p1: {res}");
}

struct RepetitionFinder {
    text: String,
}

impl RepetitionFinder {
    fn new(pat: String) -> Self {
        Self { text: pat }
    }

    fn find_pattern(&self) -> bool {
        let mut i = 1;
        let additional = if self.text.len() & 0b1 == 1 { 1 } else { 0 };
        while i != self.text.len() && i <= self.text.len() / 2 + additional {
            let a: String = self.text.chars().take(i).collect();
            let b: String = self.text.chars().skip(i).collect();
            let r = b.len() / a.len();
            if a.repeat(r) == b {
                return true;
            }
            i += 1;
        }
        false
    }
}

fn find_invalid_ids_p2(ranges: Vec<Range<usize>>) -> usize {
    ranges
        .iter()
        .map(|r| {
            let invalid = (r.start..=r.end)
                .filter(|i| {
                    let s = i.to_string();
                    let f = RepetitionFinder::new(s);
                    f.find_pattern()
                })
                .collect::<Vec<_>>();
            invalid
        })
        .flatten()
        .sum()
}

pub fn part2() {
    let file: Vec<Range<usize>> = read_file_to_vec("d2p1", ",")
        .iter()
        .map(|s| s.trim())
        .map(|s| {
            let (start, end) = s.split_once("-").unwrap();
            let start: usize = start.parse().unwrap();
            let end: usize = end.parse().unwrap();
            Range { start, end }
        })
        .collect();
    let res = find_invalid_ids_p2(file);
    println!("d2p2: {res}");
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use crate::day2::{find_invalid_ids_p1, find_invalid_ids_p2};

    fn new_range(start: usize, end: usize) -> Range<usize> {
        Range { start, end }
    }

    #[test]
    fn test_find_invalid_ids_p1() {
        let ranges = vec![
            new_range(11, 22),
            new_range(95, 115),
            new_range(998, 1012),
            new_range(1188511880, 1188511890),
            new_range(222220, 222224),
            new_range(1698522, 1698528),
            new_range(446443, 446449),
            new_range(38593856, 38593862),
        ];
        let sum = find_invalid_ids_p1(ranges);
        assert_eq!(sum, 1227775554);
    }

    #[test]
    fn test_find_invalid_ids_p2() {
        let ranges = vec![
            new_range(11, 22),
            new_range(95, 115),
            new_range(998, 1012),
            new_range(1188511880, 1188511890),
            new_range(222220, 222224),
            new_range(1698522, 1698528),
            new_range(446443, 446449),
            new_range(38593856, 38593862),
            new_range(565653, 565659),
            new_range(824824821, 824824827),
            new_range(2121212118, 2121212124),
        ];
        let sum = find_invalid_ids_p2(ranges);
        assert_eq!(sum, 4174379265);
    }
}
