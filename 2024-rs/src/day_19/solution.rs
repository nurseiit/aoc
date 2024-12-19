use std::fs::read_to_string;

use anyhow::{anyhow, Context, Ok, Result};

#[derive(Debug, PartialEq, Eq)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

type Pattern = Vec<Color>;

#[derive(Debug)]
struct InputData {
    allowed_patterns: Vec<Pattern>,
    designs: Vec<Pattern>,
}

fn string_to_pattern(pattern_str: &str) -> Result<Pattern> {
    pattern_str
        .chars()
        .map(|color| match color {
            'w' => Ok(Color::White),
            'u' => Ok(Color::Blue),
            'b' => Ok(Color::Black),
            'r' => Ok(Color::Red),
            'g' => Ok(Color::Green),
            _ => Err(anyhow!("invalid color '{}'", color)),
        })
        .collect()
}

fn read_input(file_name: &str) -> Result<InputData> {
    let input =
        read_to_string(format!("./src/day_19/{}.txt", file_name)).context("could not read file")?;

    let mut input_parts = input.split("\n\n");

    let allowed_patterns = input_parts
        .next()
        .context("could not read allowed patterns")?
        .split(", ")
        .map(string_to_pattern)
        .collect::<Result<_>>()?;

    let designs = input_parts
        .next()
        .context("could not read designs")?
        .lines()
        .map(string_to_pattern)
        .collect::<Result<_>>()?;

    Ok(InputData {
        allowed_patterns,
        designs,
    })
}

pub fn solve() -> Result<()> {
    let data = read_input("example")?;

    println!("data: {:?}", data);

    Ok(())
}
