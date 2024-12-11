use std::{fs::read_to_string, i128, iter::repeat};

use anyhow::{Context, Result};

fn get_initial_stones() -> Result<Vec<String>> {
    let input = read_to_string("./src/day_11/input.txt").context("could not read file")?;
    Ok(input
        .trim()
        .split(" ")
        .map(|stone| stone.to_string())
        .collect())
}

fn without_leading_zeroes(num_str: &str) -> &str {
    let mut i: usize = 0;
    let chars: Vec<char> = num_str.chars().collect();
    while i + 1 < chars.len() && chars[i] == '0' {
        i += 1;
    }
    num_str.get(i..).unwrap()
}

fn part_one() -> Result<()> {
    fn blink(stones: &mut Vec<String>) {
        let mut next: Vec<String> = vec![];

        stones.iter().for_each(|stone| {
            let mut new_stones = if *stone == String::from("0") {
                vec![String::from("1")]
            } else if stone.len() % 2 == 0 {
                vec![
                    stone.get(0..stone.len() / 2).unwrap().to_string(),
                    without_leading_zeroes(stone.get(stone.len() / 2..).unwrap()).to_string(),
                ]
            } else {
                vec![(stone.parse::<i128>().unwrap() * 2024).to_string()]
            };
            next.append(&mut new_stones);
        });

        *stones = next;
    }

    let mut stones = get_initial_stones()?;
    let blinks_count = 25;

    repeat(0)
        .take(blinks_count)
        .for_each(|_| blink(&mut stones));

    println!("result after {} blinks:\n{}", blinks_count, stones.len());

    Ok(())
}

pub fn solve() -> Result<()> {
    part_one()
}
