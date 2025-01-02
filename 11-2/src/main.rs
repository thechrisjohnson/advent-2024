use std::{fmt, io::Read, sync::mpsc, thread};

fn main() {
    let input = get_input().unwrap();
    let stones = get_stones(&input);

    let mut total = 0;

    let (tx, rx) = mpsc::channel();
    for stone in stones {
        let tx1 = tx.clone();
        thread::spawn(move || {
            let x = blink(stone, 1, DEFAULT_BLINKS);
            tx1.send(x).unwrap();
        });
    }

    drop(tx);

    while let Ok(result) = rx.recv() {
        println!("Result {}", &result);
        total += result;
    }

    println!(
        "After {} blinks, we have the following stones: {}",
        DEFAULT_BLINKS, total,
    );
}

fn get_stones(input: &str) -> Vec<usize> {
    let mut stones = Vec::new();

    for line in input.lines() {
        for number in line.split(' ') {
            if !number.is_empty() {
                stones.push(str::parse::<usize>(number).unwrap());
            }
        }
    }

    stones
}

fn get_input() -> Result<String, Error> {
    let mut input = String::new();
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut input)?;

    if input.is_empty() {
        input = DEFAULT_INPUT.to_string();
    }

    Ok(input)
}

fn blink(number: usize, current_iteration: usize, max_iterations: usize) -> usize {
    if current_iteration == max_iterations {
        if num_digits(number) % 2 == 0 {
            2
        } else {
            1
        }
    } else if number == 0 {
        return blink(1, current_iteration + 1, max_iterations);
    } else {
        let num_digits = num_digits(number);
        if num_digits % 2 == 0 {
            let divisor = TEN.pow(num_digits / 2);
            return blink(number / divisor, current_iteration + 1, max_iterations)
                + blink(number % divisor, current_iteration + 1, max_iterations);
        } else {
            return blink(
                number * STONE_MULTIPLIER,
                current_iteration + 1,
                max_iterations,
            );
        }
    }
}

fn num_digits(number: usize) -> u32 {
    let mut num_digits = 1;
    let mut modulus = 10;
    loop {
        if number % modulus == number {
            return num_digits;
        }

        modulus *= 10;
        num_digits += 1;
    }
}

#[derive(Debug)]
struct Error {
    message: String,
}

impl Error {
    fn new(message: String) -> Self {
        Error { message }
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::new(value.to_string())
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(value: std::num::ParseIntError) -> Self {
        Error::new(value.to_string())
    }
}

const TEN: usize = 10;
const DEFAULT_BLINKS: usize = 75;
const STONE_MULTIPLIER: usize = 2024;

const DEFAULT_INPUT: &str = "8435 234 928434 14 0 7 92446 8992692";
