use std::{collections::HashMap, fs::read_to_string};

use anyhow::{Context, Result};
use itertools::Itertools;

fn next_secret(mut secret: i64) -> i64 {
    fn prune(num: i64) -> i64 {
        num % 16777216
    }

    fn mix(secret: i64, num: i64) -> i64 {
        num ^ secret
    }

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

fn read_secrets(file_name: &str) -> Result<Vec<i64>> {
    let input =
        read_to_string(format!("./src/day_22/{}.txt", file_name)).context("could not read file")?;
    input
        .lines()
        .map(|line| line.parse::<i64>().context("could not parse line"))
        .collect()
}

fn part_one() -> Result<()> {
    let secrets = read_secrets("input")?;

    let result = secrets
        .iter()
        .map(|&secret| (0..2000).fold(secret, |acc, _| next_secret(acc)))
        .sum::<i64>();

    println!("part one result: {}", result);

    Ok(())
}

fn part_two() -> Result<()> {
    let secrets = read_secrets("input")?;

    let mut cache_per_secret: Vec<HashMap<_, i64>> = vec![];

    secrets.iter().enumerate().for_each(|(secret_i, &secret)| {
        cache_per_secret.push(HashMap::new());

        let mut seq = vec![];
        let mut secret_inner = secret;

        (0..2000).into_iter().for_each(|_| {
            seq.push(secret_inner % 10);
            secret_inner = next_secret(secret_inner);
        });

        seq.windows(2)
            .map(|window| window[0] - window[1])
            .tuple_windows::<(_, _, _, _)>()
            .enumerate()
            .for_each(|(i, key)| {
                if cache_per_secret[secret_i].contains_key(&key) {
                    return;
                }
                let val = seq[i + 4];
                cache_per_secret[secret_i].insert(key, val);
            });
    });

    let mut results_per_quarter = HashMap::new();

    cache_per_secret.iter().for_each(|cache| {
        cache.iter().for_each(|(&key, &val)| {
            *results_per_quarter.entry(key).or_insert(0) += val;
        });
    });

    println!(
        "part two result: {}",
        results_per_quarter.into_values().max().unwrap()
    );

    Ok(())
}

pub fn solve() -> Result<()> {
    part_one()?;
    part_two()
}
