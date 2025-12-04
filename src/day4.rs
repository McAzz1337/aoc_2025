use crate::read_file_to_vec;
fn mark_position(rolls: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let start_x = x.saturating_sub(1);
    let end_x = (x + 1).min(rolls[y].len() - 1);
    let start_y = y.saturating_sub(1);
    let end_y = (y + 1).min(rolls.len() - 1);

    (start_y..=end_y)
        .map(|iy| {
            (start_x..=end_x)
                .map(|ix| {
                    if ix == x && iy == y {
                        false
                    } else if rolls[iy][ix] == '@' {
                        true
                    } else {
                        false
                    }
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .filter(|b| *b)
        .count()
        < 4
}

fn mark_all_reachables(rolls: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mark = |c: &char, x: usize, y: usize| match c {
        '.' => '.',
        _ => {
            if mark_position(rolls, x, y) {
                'x'
            } else {
                *c
            }
        }
    };
    rolls
        .iter()
        .enumerate()
        .map(|(y, v)| {
            v.iter()
                .enumerate()
                .map(|(x, c)| mark(c, x, y))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn count_all_reachables(rolls: &Vec<Vec<char>>) -> usize {
    mark_all_reachables(rolls)
        .iter()
        .flatten()
        .filter(|c| **c == 'x')
        .count()
}

pub fn part1() {
    let file: Vec<Vec<_>> = read_file_to_vec("d4p1", "\n")
        .iter()
        .map(|s| s.as_str().chars().collect::<Vec<_>>())
        .collect();
    let res = count_all_reachables(&file);
    println!("d4p1: {res}");
}

fn remove_all_reachables(rolls: &Vec<Vec<char>>) -> (Vec<Vec<char>>, usize) {
    let mut removed = 0;
    let res: Vec<Vec<char>> = rolls
        .iter()
        .map(|v| {
            v.iter()
                .map(|c| {
                    if *c == 'x' {
                        removed += 1;
                        '.'
                    } else {
                        *c
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();
    (res, removed)
}

pub fn part2() {
    let mut file: Vec<Vec<_>> = read_file_to_vec("d4p1", "\n")
        .iter()
        .map(|s| s.as_str().chars().collect::<Vec<_>>())
        .collect();
    let mut changed = 0;
    let mut running = true;
    while running {
        file = mark_all_reachables(&file);
        let (f, c) = remove_all_reachables(&file);
        file = f;
        changed += c;
        running = c > 0;
    }
    println!("d4p2: {changed}");
}

#[cfg(test)]
mod tests {
    use crate::day4::{count_all_reachables, mark_all_reachables, remove_all_reachables};

    #[test]
    fn test_count_all_reachable() {
        let input = vec![
            "..@@.@@@@.",
            "@@@.@.@.@@",
            "@@@@@.@.@@",
            "@.@@@@..@.",
            "@@.@@@@.@@",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "@.@@@.@@@@",
            ".@@@@@@@@.",
            "@.@.@@@.@.",
        ]
        .iter()
        .map(|v| v.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

        let res = count_all_reachables(&input);
        assert_eq!(res, 13);
    }

    #[test]
    fn test_remove_all_reachables() {
        let mut input = vec![
            "..@@.@@@@.",
            "@@@.@.@.@@",
            "@@@@@.@.@@",
            "@.@@@@..@.",
            "@@.@@@@.@@",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "@.@@@.@@@@",
            ".@@@@@@@@.",
            "@.@.@@@.@.",
        ]
        .iter()
        .map(|v| v.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

        let mut changed = 0;
        let mut running = true;
        while running {
            input = mark_all_reachables(&input);
            let (i, c) = remove_all_reachables(&input);
            input = i;
            changed += c;
            running = c > 0;
        }
        assert_eq!(changed, 43);
    }
}
