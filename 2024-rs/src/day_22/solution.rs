use std::fs::read_to_string;

use anyhow::{Context, Result};

fn prune(num: u64) -> u64 {
    num % 16777216
}

fn mix(secret: u64, num: u64) -> u64 {
    num ^ secret
}

fn next_secret(mut secret: u64) -> u64 {
    let first = secret * 64;
    secret = mix(secret, first);
    secret = prune(secret);

    let second = secret >> 5;
    secret = mix(secret, second);
    secret = prune(secret);

    let third = secret << 11;
    secret = mix(secret, third);
    secret = prune(secret);

    secret
}

fn read_secrets(file_name: &str) -> Result<Vec<u64>> {
    let input =
        read_to_string(format!("./src/day_22/{}.txt", file_name)).context("could not read file")?;
    input
        .lines()
        .map(|line| line.parse::<u64>().context("could not parse line"))
        .collect()
}

pub fn solve() -> Result<()> {
    let secrets = read_secrets("input")?;

    let result = secrets
        .iter()
        .map(|&secret| (0..2000).fold(secret, |acc, _| next_secret(acc)))
        .sum::<u64>();

    println!("part one result: {}", result);

    Ok(())
}
