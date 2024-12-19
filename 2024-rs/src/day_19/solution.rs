use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

use anyhow::{anyhow, Context, Ok, Result};

#[derive(Debug, PartialEq, Eq, Hash)]
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
    allowed_patterns: HashSet<Pattern>,
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
    fn count_possible_ways_for_design(
        design: &Pattern,
        index: usize,
        allowed_patterns: &HashSet<Pattern>,
        cache: &mut HashMap<usize, u64>,
    ) -> u64 {
        let len = design.len();

        if index == len {
            return 1;
        }

        if let Some(cached_result) = cache.get(&index) {
            return *cached_result;
        }

        let result = allowed_patterns
            .iter()
            .filter(|pattern| pattern.len() + index <= len)
            .filter(|pattern| (0..pattern.len()).all(|i| pattern[i] == design[index + i]))
            .map(|pattern| {
                count_possible_ways_for_design(
                    design,
                    index + pattern.len(),
                    allowed_patterns,
                    cache,
                )
            })
            .sum();

        cache.insert(index, result);

        result
    }

    let InputData {
        allowed_patterns,
        designs,
    } = read_input("input")?;

    let results: Vec<u64> = designs
        .iter()
        .map(|design| {
            let mut cache = HashMap::new();
            count_possible_ways_for_design(design, 0, &allowed_patterns, &mut cache)
        })
        .collect();

    println!(
        "part one result: {}",
        results.iter().filter(|&result| *result > 0).count()
    );

    println!("part two result: {}", results.iter().sum::<u64>());

    Ok(())
}
