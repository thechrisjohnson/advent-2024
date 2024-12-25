use std::{fmt, io::Read};

fn main() {
    let input = get_input().unwrap();

    println!("input: {}", &input);
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

const DEFAULT_INPUT: &str = "DEFAULT INPUT";
