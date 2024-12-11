use std::{collections::HashMap, fs::read_to_string, i128, iter::repeat};

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

fn transform_stone(stone: &String) -> Vec<String> {
    if *stone == String::from("0") {
        vec![String::from("1")]
    } else if stone.len() % 2 == 0 {
        vec![
            stone.get(0..stone.len() / 2).unwrap().to_string(),
            without_leading_zeroes(stone.get(stone.len() / 2..).unwrap()).to_string(),
        ]
    } else {
        vec![(stone.parse::<i128>().unwrap() * 2024).to_string()]
    }
}

fn part_one(blinks_count: usize) -> Result<()> {
    let mut stones = get_initial_stones()?;

    fn blink(stones: &mut Vec<String>) {
        let mut next: Vec<String> = vec![];

        stones.iter().for_each(|stone| {
            let mut new_stones = transform_stone(stone);
            next.append(&mut new_stones);
        });

        *stones = next;
    }

    repeat(0)
        .take(blinks_count)
        .for_each(|_| blink(&mut stones));

    println!("result after {} blinks:\n{}", blinks_count, stones.len());

    Ok(())
}

fn part_two(blinks_count: i32) -> Result<()> {
    fn count_after_blink(
        stone: &String,
        count: i32,
        cache: &mut HashMap<String, HashMap<i32, i128>>,
    ) -> i128 {
        if count == 0 {
            return 1;
        }
        if let Some(top) = cache.get(stone) {
            if let Some(result) = top.get(&count) {
                return *result;
            }
        }
        let next = transform_stone(stone);
        let result: i128 = next
            .iter()
            .map(|next_stone| count_after_blink(next_stone, count - 1, cache))
            .sum();
        cache
            .entry(stone.clone())
            .or_insert(HashMap::new())
            .insert(count, result);
        result
    }

    let stones = get_initial_stones()?;
    let mut cache: HashMap<String, HashMap<i32, i128>> = HashMap::new();

    let result: i128 = stones
        .iter()
        .map(|stone| count_after_blink(stone, blinks_count, &mut cache))
        .sum();

    println!("result after {} blinks:\n{}", blinks_count, result);

    Ok(())
}

pub fn solve() -> Result<()> {
    let blinks_count = 75;
    // let _ = part_one(blinks_count as usize)?;
    let _ = part_two(blinks_count)?;
    Ok(())
}
