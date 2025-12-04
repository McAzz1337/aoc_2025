use crate::read_file_to_vec;

fn find_largest_two_digits(pack: &str) -> (usize, usize) {
    let digits: Vec<_> = pack
        .chars()
        .enumerate()
        .map(|(i, c)| (i, c.to_digit(10).unwrap() as usize))
        .collect();
    let largest = digits
        .iter()
        .take(digits.len() - 1)
        .rev()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();
    let second_largest = digits
        .iter()
        .skip(largest.0 + 1)
        .rev()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();

    (largest.1, second_largest.1)
}

fn assemble_digits(a: usize, b: usize) -> usize {
    a * 10 + b
}

fn turn_on_batteries(battery_pack: Vec<String>) -> usize {
    battery_pack
        .iter()
        .map(|b| find_largest_two_digits(b))
        .map(|(a, b)| assemble_digits(a, b))
        .sum()
}

pub fn part1() {
    let file: Vec<_> = read_file_to_vec("d3p1", "\n")
        .iter()
        .map(|s| s.to_string())
        .collect();
    let res = turn_on_batteries(file);
    println!("d3p1: {res}");
}

fn find_12_largest_digits(pack: &str, i: usize) -> Vec<usize> {
    if i >= 12 {
        vec![]
    } else {
        let last_index = pack.len() - (12 - i) + 1;
        let considered = &pack[0..last_index];
        let digits: Vec<_> = considered
            .chars()
            .enumerate()
            .map(|(i, c)| (i, c.to_digit(10).unwrap() as usize))
            .collect();
        let largest = digits
            .iter()
            .rev()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap();

        [
            vec![largest.1],
            find_12_largest_digits(&pack[largest.0 + 1..], i + 1),
        ]
        .concat()
    }
}

fn assemble_12_digits(digits: Vec<usize>) -> usize {
    digits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, x)| x * (10_usize.pow(i as u32)))
        .sum()
}

fn turn_on_12_batteries(battery_pack: Vec<String>) -> usize {
    battery_pack
        .iter()
        .map(|b| find_12_largest_digits(b, 0))
        .map(|v| assemble_12_digits(v))
        .sum()
}
pub fn part2() {
    let file: Vec<_> = read_file_to_vec("d3p1", "\n")
        .iter()
        .map(|s| s.to_string())
        .collect();
    let res = turn_on_12_batteries(file);
    println!("d3p2: {res}");
}

#[cfg(test)]
mod tests {
    use crate::day3::{
        assemble_12_digits, assemble_digits, find_12_largest_digits, find_largest_two_digits,
        turn_on_12_batteries, turn_on_batteries,
    };

    #[test]
    fn test_turn_on_batteris() {
        let battery_packs = vec![
            ("987654321111111".to_string(), 98_usize),
            ("811111111111119".to_string(), 89_usize),
            ("234234234234278".to_string(), 78_usize),
            ("818181911112111".to_string(), 92_usize),
        ];

        battery_packs.iter().for_each(|(p, res)| {
            let (a, b) = find_largest_two_digits(p);
            assert_eq!(assemble_digits(a, b), *res);
        });
        let sum = turn_on_batteries(battery_packs.iter().map(|p| p.0.clone()).collect());
        assert_eq!(sum, 357);
    }

    #[test]
    fn test_part2() {
        let battery_packs = vec![
            ("987654321111111".to_string(), 987654321111_usize),
            ("811111111111119".to_string(), 811111111119_usize),
            ("234234234234278".to_string(), 434234234278_usize),
            ("818181911112111".to_string(), 888911112111_usize),
        ];
        battery_packs.iter().for_each(|(p, res)| {
            let v = find_12_largest_digits(p, 0);
            assert_eq!(assemble_12_digits(v), *res);
        });
        let sum = turn_on_12_batteries(battery_packs.iter().map(|p| p.0.clone()).collect());
        assert_eq!(sum, 3121910778619);
    }
}
