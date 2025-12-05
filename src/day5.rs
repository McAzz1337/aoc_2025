use std::ops::{Range, RangeInclusive};

use crate::read_file_to_vec;

fn string_to_range(s: &str) -> Result<RangeInclusive<usize>, String> {
    match s.split_once("-") {
        Some((a, b)) => {
            let start = a.parse::<usize>().unwrap();
            let end = b.parse::<usize>().unwrap();
            Ok(RangeInclusive::new(start, end))
        }
        None => Err(format!("Failed to split at \"-\": value")),
    }
}

fn count_fresh_ingredients(ranges: &Vec<RangeInclusive<usize>>, ids: &Vec<usize>) -> usize {
    ids.iter()
        .filter(|i| ranges.iter().any(|r| r.contains(i)))
        .count()
}

pub fn part1() {
    let file: Vec<_> = read_file_to_vec("d5p1", "\n");
    let ranges: Vec<_> = file
        .iter()
        .filter(|s| s.contains("-"))
        .flat_map(|s| string_to_range(s))
        .collect();
    let ids: Vec<_> = file
        .iter()
        .filter(|s| !s.contains("-"))
        .flat_map(|s| s.parse::<usize>())
        .collect();

    let res = count_fresh_ingredients(&ranges, &ids);
    println!("d5p1: {res}");
}

fn ranges_overlap(a: &RangeInclusive<usize>, b: &RangeInclusive<usize>) -> bool {
    a.contains(&b.start()) || a.contains(&b.end()) || b.contains(&a.start()) || b.contains(&a.end())
}

fn merge_ranges(a: &RangeInclusive<usize>, b: &RangeInclusive<usize>) -> RangeInclusive<usize> {
    let start = a.start().min(b.start());
    let end = a.end().max(b.end());
    RangeInclusive::new(*start, *end)
}

pub fn count_fresh_ids_in_ranges(ranges: &Vec<RangeInclusive<usize>>) -> usize {
    ranges.iter().map(|r| r.end() - r.start() + 1).sum()
}

pub fn part2() {
    let file: Vec<_> = read_file_to_vec("d5p1", "\n");
    let mut ranges: Vec<_> = file
        .iter()
        .filter(|s| s.contains("-"))
        .flat_map(|s| string_to_range(s))
        .collect();

    let mut i = 0;
    while i < ranges.len() {
        let mut merged = false;
        while let Some((j, r)) = ranges
            .iter()
            .enumerate()
            .skip(i + 1)
            .find(|(_, r)| ranges_overlap(&ranges[i], r))
        {
            ranges[i] = merge_ranges(&ranges[i], r);
            ranges.remove(j);
            merged = true;
        }
        if !merged {
            i += 1;
        }
    }

    let res = count_fresh_ids_in_ranges(&ranges);
    println!("d5p2: {res}");
}

#[cfg(test)]
mod tests {
    use std::ops::{Range, RangeInclusive};

    use crate::day5::{
        count_fresh_ids_in_ranges, count_fresh_ingredients, merge_ranges, ranges_overlap,
        string_to_range,
    };

    #[test]
    fn test_count_fresh_ingredients() {
        let input: Vec<_> = vec![
            "3-5", "10-14", "16-20", "12-18", "1", "5", "8", "11", "17", "32",
        ];

        let ranges: Vec<_> = input
            .iter()
            .filter(|s| s.contains("-"))
            .flat_map(|s| string_to_range(s))
            .collect();
        let ids: Vec<_> = input
            .iter()
            .filter(|s| !s.contains("-"))
            .flat_map(|s| s.parse::<usize>())
            .collect();

        let res = count_fresh_ingredients(&ranges, &ids);
        assert_eq!(res, 3);
    }

    #[test]
    fn test_ranges_overlap() {
        let a = RangeInclusive::new(10, 20);
        let b = RangeInclusive::new(15, 25);
        let c = RangeInclusive::new(100, 200);
        let d = RangeInclusive::new(20, 30);
        assert!(ranges_overlap(&a, &b));
        assert!(!ranges_overlap(&a, &c));
        assert!(ranges_overlap(&a, &d));
    }

    #[test]
    fn test_merge_ranges() {
        let a = RangeInclusive::new(10, 20);
        let b = RangeInclusive::new(15, 25);
        assert_eq!(merge_ranges(&a, &b), RangeInclusive::new(10, 25));
        let c = RangeInclusive::new(20, 30);
        assert_eq!(merge_ranges(&a, &c), RangeInclusive::new(10, 30));
    }

    #[test]
    fn test_count_fresh_ingredients_in_ranges() {
        let input: Vec<_> = vec![
            "3-5", "10-14", "16-20", "12-18", "1", "5", "8", "11", "17", "32",
        ];

        let mut ranges: Vec<_> = input
            .iter()
            .filter(|s| s.contains("-"))
            .flat_map(|s| string_to_range(s))
            .collect();

        let mut i = 0;
        while i < ranges.len() {
            let mut merged = false;
            while let Some((j, r)) = ranges
                .iter()
                .enumerate()
                .skip(i + 1)
                .find(|(_, r)| ranges_overlap(&ranges[i], r))
            {
                ranges[i] = merge_ranges(&ranges[i], r);
                ranges.remove(j);
                merged = true;
            }
            if !merged {
                i += 1;
            }
        }

        assert_eq!(
            ranges,
            vec![RangeInclusive::new(3, 5), RangeInclusive::new(10, 20)]
        );
        let res = count_fresh_ids_in_ranges(&ranges);
        assert_eq!(res, 14);
    }
}
