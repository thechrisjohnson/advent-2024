use std::{collections::HashMap, fmt, io::Read};

fn main() {
    let input = get_input().unwrap();
    let stones = get_stones(&input);

    let mut total = 0;
    let mut cache = HashMap::new();

    for stone in stones {
        total += blink(&mut cache, stone, 1, DEFAULT_BLINKS);
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

fn blink(
    cache: &mut HashMap<(usize, usize), usize>,
    number: usize,
    current_iteration: usize,
    max_iterations: usize,
) -> usize {
    if current_iteration == max_iterations {
        if num_digits(number) % 2 == 0 {
            2
        } else {
            1
        }
    } else if let Some(x) = cache.get(&(number, current_iteration)) {
        return *x;
    } else if number == 0 {
        let result = blink(cache, 1, current_iteration + 1, max_iterations);
        cache.insert((number, current_iteration), result);
        return result;
    } else {
        let num_digits = num_digits(number);
        if num_digits % 2 == 0 {
            let divisor = TEN.pow(num_digits / 2);
            let result = blink(
                cache,
                number / divisor,
                current_iteration + 1,
                max_iterations,
            ) + blink(
                cache,
                number % divisor,
                current_iteration + 1,
                max_iterations,
            );
            cache.insert((number, current_iteration), result);
            return result;
        } else {
            let result = blink(
                cache,
                number * STONE_MULTIPLIER,
                current_iteration + 1,
                max_iterations,
            );
            cache.insert((number, current_iteration), result);
            return result;
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
