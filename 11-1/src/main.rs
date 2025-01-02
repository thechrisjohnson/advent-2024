use std::{fmt, io::Read};

fn main() {
    let input = get_input().unwrap();
    let mut stones = get_stones(&input).unwrap();
    print_stones(0, &stones);
    for i in 1..=DEFAULT_BLINKS {
        stones = blink(stones).unwrap();
        print_stones(i, &stones);
    }

    println!(
        "After {} blinks, we have the following stones: {}",
        DEFAULT_BLINKS,
        stones.len()
    );
}

fn blink(stones: Vec<Stone>) -> Result<Vec<Stone>, Error> {
    let mut new_stones = Vec::new();
    for stone in stones {
        new_stones.append(&mut stone.blink()?);
    }

    Ok(new_stones)
}

fn get_stones(input: &str) -> Result<Vec<Stone>, Error> {
    let mut stones = Vec::new();

    for line in input.lines() {
        for number in line.split(' ') {
            if !number.is_empty() {
                stones.push(Stone::parse(number)?);
            }
        }
    }

    Ok(stones)
}

fn print_stones(blink_count: i32, stones: &Vec<Stone>) {
    let mut output = format!("{}:", blink_count);
    for stone in stones {
        output = format!("{} {}", &output, stone.number);
    }

    println!("{}", &output);
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

struct Stone {
    number: usize,
}

impl Stone {
    fn parse(input: &str) -> Result<Self, Error> {
        Ok(Self {
            number: str::parse::<usize>(input)?,
        })
    }

    fn blink(&self) -> Result<Vec<Self>, Error> {
        if self.number == 0 {
            Ok(vec![Self { number: 1 }])
        } else if self.number.to_string().len() % 2 == 0 {
            let mut first_number = self.number.to_string();
            let second_number = first_number.split_off(first_number.len() / 2);
            Ok(vec![
                Self {
                    number: str::parse::<usize>(&first_number)?,
                },
                Self {
                    number: str::parse::<usize>(&second_number)?,
                },
            ])
        } else {
            Ok(vec![Self {
                number: self.number * STONE_MULTIPLIER,
            }])
        }
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

const DEFAULT_BLINKS: i32 = 25;
const STONE_MULTIPLIER: usize = 2024;

const DEFAULT_INPUT: &str = "8435 234 928434 14 0 7 92446 8992692";
