use std::io::Read;

const DEFAULT_INPUT: &str = "DEFAULT INPUT";

fn main() {
    let input = get_input().unwrap();

    println!("input: {}", &input);
}

fn get_input() -> Result<String, std::io::Error> {
    let mut input = String::new();
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut input)?;

    if input.is_empty() {
        input = DEFAULT_INPUT.to_string();
    }

    Ok(input)
}
