use std::{fs, thread};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn read_file_to_vec(path: &str, delimeter: &str) -> Vec<String> {
    let path = String::from("puzzles/") + path + ".txt";
    fs::read_to_string(&path)
        .expect(&format!("Failed to read file: {path}"))
        .split(delimeter)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_owned())
        .collect()
}

fn main() {
    let funcs = vec![
        day1::part1,
        day1::part2,
        day2::part1,
        day2::part2,
        day3::part1,
        day3::part2,
        day4::part1,
        day4::part2,
        day5::part1,
        day5::part2,
    ];

    let threads: Vec<_> = funcs.iter().map(|f| thread::spawn(f.clone())).collect();

    for t in threads {
        t.join().unwrap()
    }
}
