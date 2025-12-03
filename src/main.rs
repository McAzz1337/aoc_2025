use std::thread;

mod day1;
mod day2;
mod day3;

fn main() {
    let funcs = vec![
        day1::part1,
        day1::part2,
        day2::part1,
        day2::part2,
        day3::part1,
        day3::part2,
    ];

    let threads: Vec<_> = funcs.iter().map(|f| thread::spawn(f.clone())).collect();

    for t in threads {
        t.join().unwrap()
    }
}
