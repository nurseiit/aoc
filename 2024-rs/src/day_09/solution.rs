use std::{fs::read_to_string, iter::repeat};

use anyhow::{Context, Result};

fn get_disk_map() -> Result<Vec<i32>> {
    let input = read_to_string("./src/day_09/input.txt").context("could not read file")?;
    input
        .trim()
        .chars()
        .map(|char| char.to_digit(10).context("could not convert to digit"))
        .map(|num| Ok(num? as i32))
        .collect()
}

fn get_disk_blocks() -> Result<Vec<i32>> {
    let disk_map = get_disk_map()?;
    let mut blocks: Vec<i32> = vec![];

    disk_map.iter().enumerate().for_each(|(i, &num)| {
        repeat(if i % 2 == 0 { i as i32 / 2 } else { -1 })
            .take(num as usize)
            .for_each(|block| blocks.push(block))
    });

    Ok(blocks)
}

fn part_one() -> Result<()> {
    let mut blocks = get_disk_blocks()?;

    let mut left = 0usize;
    let mut right = blocks.len() - 1;

    loop {
        while left < right && blocks[left] != -1 {
            left += 1;
        }
        while left < right && blocks[right] == -1 {
            right -= 1;
        }
        if left >= right {
            break;
        }
        (blocks[left], blocks[right]) = (blocks[right], blocks[left]);
    }

    let result: i64 = blocks
        .iter()
        .enumerate()
        .map(|(i, &val)| i as i64 * val as i64)
        .filter(|&val| val > 0)
        .sum();

    println!("part one result {}", result);

    Ok(())
}

pub fn solve() -> Result<()> {
    part_one()
}
