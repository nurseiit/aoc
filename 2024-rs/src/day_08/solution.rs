use anyhow::{Context, Result};
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn read_table_from_file(file_path: &str) -> Result<Vec<Vec<char>>> {
    let input = read_to_string(file_path).context("could not read file")?;
    let table = input.lines().map(|line| line.chars().collect()).collect();
    Ok(table)
}

fn part_one() -> Result<()> {
    let table = read_table_from_file("./src/day_08/input.txt")?;

    let n = table.len() as i32;
    let m = table.get(1).context("could not get the first row")?.len() as i32;

    let mut chars: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    table.iter().enumerate().for_each(|(i, row)| {
        row.iter()
            .enumerate()
            .filter(|(_, char)| char.is_ascii_alphanumeric())
            .for_each(|(j, char)| {
                chars
                    .entry(*char)
                    .or_insert(vec![])
                    .push((i as i32, j as i32))
            })
    });

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    chars.iter().for_each(|(_, positions)| {
        positions.iter().for_each(|&first| {
            positions.iter().for_each(|&second| {
                let (dx, dy) = (first.0 - second.0, first.1 - second.1);
                if dx == 0 && dy == 0 {
                    return;
                }
                let (ni, nj) = (first.0 + dx, first.1 + dy);
                if ni < 0 || ni >= n || nj < 0 || nj >= m {
                    return;
                }
                antinodes.insert((ni, nj));
            })
        });
    });

    println!("part one result {}", antinodes.len());

    Ok(())
}

fn part_two() -> Result<()> {
    let table = read_table_from_file("./src/day_08/input.txt")?;

    let n = table.len() as i32;
    let m = table.get(1).context("could not get the first row")?.len() as i32;

    let mut chars: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    table.iter().enumerate().for_each(|(i, row)| {
        row.iter()
            .enumerate()
            .filter(|(_, char)| char.is_ascii_alphanumeric())
            .for_each(|(j, char)| {
                chars
                    .entry(*char)
                    .or_insert(vec![])
                    .push((i as i32, j as i32))
            })
    });

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    chars.iter().for_each(|(_, positions)| {
        positions.iter().for_each(|&first| {
            // insert itself
            antinodes.insert(first);

            // loop through all produced antinodes with other chars
            positions.iter().for_each(|&second| {
                let (dx, dy) = (first.0 - second.0, first.1 - second.1);
                if dx == 0 && dy == 0 {
                    return;
                }

                let mut ni = first.0 + dx;
                let mut nj = first.1 + dy;
                loop {
                    if ni < 0 || ni >= n || nj < 0 || nj >= m {
                        break;
                    }
                    antinodes.insert((ni, nj));
                    ni += dx;
                    nj += dy;
                }
            })
        });
    });

    println!("part two result {}", antinodes.len());

    Ok(())
}

pub fn solve() -> Result<()> {
    part_two()
}
