use crate::read_file_to_vec;

struct Dial {
    pointer: i16,
    password: u64,
    count_end: bool,
}

impl Dial {
    const MAX: i16 = 99;
    const DELTA: i16 = 100;

    fn new(count_end: bool) -> Self {
        Self {
            pointer: 50,
            password: 0,
            count_end,
        }
    }

    fn nudge_right(&mut self) {
        self.pointer += 1;
        if self.pointer > Self::MAX {
            self.pointer -= Self::DELTA;
        }
        if !self.count_end && self.pointer == 0 {
            self.password += 1;
        }
    }

    fn turn_right(&mut self, n: i16) {
        (0..n).for_each(|_| self.nudge_right());
        if self.count_end && self.pointer == 0 {
            self.password += 1;
        }
    }

    fn nudge_left(&mut self) {
        self.pointer -= 1;
        if self.pointer < 0 {
            self.pointer += Self::DELTA;
        }
        if !self.count_end && self.pointer == 0 {
            self.password += 1;
        }
    }

    fn turn_left(&mut self, n: i16) {
        (0..n).for_each(|_| self.nudge_left());
        if self.count_end && self.pointer == 0 {
            self.password += 1;
        }
    }
}

fn turn_dial_left(dial: &mut Dial, n: i16) {
    dial.turn_left(n);
}

fn turn_dial_right(dial: &mut Dial, n: i16) {
    dial.turn_right(n);
}

pub fn part1() {
    let file = read_file_to_vec("d1p1", "\n");

    let mut dial = Dial::new(true);
    file.iter()
        .map(|s| {
            let turn_fn = if s.chars().nth(0).unwrap() == 'R' {
                turn_dial_right
            } else {
                turn_dial_left
            };
            let amount = s
                .chars()
                .skip(1)
                .collect::<String>()
                .parse::<i16>()
                .unwrap();
            // println!("{}{}", s.chars().nth(0).unwrap(), amount);
            (turn_fn, amount)
        })
        .for_each(|(turn_fn, amount)| turn_fn(&mut dial, amount));
    println!("d1p1: {}", dial.password);
}

pub fn part2() {
    let file = read_file_to_vec("d1p1", "\n");

    let mut dial = Dial::new(false);
    file.iter()
        .map(|s| {
            let turn_fn = if s.chars().nth(0).unwrap() == 'R' {
                turn_dial_right
            } else {
                turn_dial_left
            };
            let amount = s
                .chars()
                .skip(1)
                .collect::<String>()
                .parse::<i16>()
                .unwrap();
            // println!("{}{}", s.chars().nth(0).unwrap(), amount);
            (turn_fn, amount)
        })
        .for_each(|(turn_fn, amount)| turn_fn(&mut dial, amount));
    println!("d1p2: {}", dial.password);
}

#[cfg(test)]
mod tests {
    use crate::day1::Dial;

    #[test]
    fn test_dial() {
        let mut dial = Dial::new(true);
        dial.pointer = 99;
        dial.turn_right(1);
        assert_eq!(dial.pointer, 0);
        dial.turn_left(1);
        assert_eq!(dial.pointer, 99);
        dial.turn_right(6);
        assert_eq!(dial.pointer, 5);
        dial.turn_left(10);
        assert_eq!(dial.pointer, 95);
    }

    #[test]
    fn test_part1() {
        let mut dial = Dial::new(true);
        dial.turn_left(68);
        println!("{}", dial.pointer);
        dial.turn_left(30);
        println!("{}", dial.pointer);
        dial.turn_right(48);
        println!("{}", dial.pointer);
        dial.turn_left(5);
        println!("{}", dial.pointer);
        dial.turn_right(60);
        println!("{}", dial.pointer);
        dial.turn_left(55);
        println!("{}", dial.pointer);
        dial.turn_left(1);
        println!("{}", dial.pointer);
        dial.turn_left(99);
        println!("{}", dial.pointer);
        dial.turn_right(14);
        println!("{}", dial.pointer);
        dial.turn_left(82);
        println!("{}", dial.pointer);
        assert_eq!(dial.password, 3);
    }

    #[test]
    fn test_part2() {
        let mut dial = Dial::new(false);
        dial.turn_left(68);
        println!("{}", dial.pointer);
        dial.turn_left(30);
        println!("{}", dial.pointer);
        dial.turn_right(48);
        println!("{}", dial.pointer);
        dial.turn_left(5);
        println!("{}", dial.pointer);
        dial.turn_right(60);
        println!("{}", dial.pointer);
        dial.turn_left(55);
        println!("{}", dial.pointer);
        dial.turn_left(1);
        println!("{}", dial.pointer);
        dial.turn_left(99);
        println!("{}", dial.pointer);
        dial.turn_right(14);
        println!("{}", dial.pointer);
        dial.turn_left(82);
        println!("{}", dial.pointer);
        assert_eq!(dial.password, 6);
    }
}
