use std::{collections::HashSet, fs::read_to_string, iter::repeat};

use anyhow::{Context, Result};

fn get_map_from_input() -> Result<Vec<Vec<i32>>> {
    let input = read_to_string("./src/day_10/input.txt").context("could not read file")?;
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| {
                    let val = char.to_digit(10).context("could not convert to digit")?;
                    Ok(val as i32)
                })
                .collect()
        })
        .collect()
}

fn get_positions_with_value(value: i32, map: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    map.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &cell)| cell == value)
                .map(|(j, _)| (i, j))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect()
}

fn part_one() -> Result<()> {
    let map = get_map_from_input()?;
    let n = map.len() as i32;
    let m = map
        .get(0)
        .context("could not get the first item in map")?
        .len() as i32;

    let mut result: Vec<Vec<HashSet<(usize, usize)>>> =
        repeat(repeat(HashSet::new()).take(m as usize).collect())
            .take(n as usize)
            .collect();

    get_positions_with_value(9, &map)
        .iter()
        .for_each(|&(i, j)| {
            result[i][j].insert((i, j));
        });

    let dxdy: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    (0..9).rev().for_each(|step| {
        get_positions_with_value(step, &map)
            .iter()
            .for_each(|&(i, j)| {
                let step_neighbours: Vec<(usize, usize)> = dxdy
                    .iter()
                    .map(|&(dx, dy)| (i as i32 + dx, j as i32 + dy))
                    .filter(|&(ni, nj)| 0 <= ni && ni < n && 0 <= nj && nj < m)
                    .map(|(ni, nj)| (ni as usize, nj as usize))
                    .filter(|&(ni, nj)| map[ni][nj] == step + 1)
                    .collect();
                step_neighbours.iter().for_each(|&(ni, nj)| {
                    for pos in result[ni][nj].clone() {
                        result[i][j].insert(pos);
                    }
                });
            })
    });

    let result: i32 = get_positions_with_value(0, &map)
        .iter()
        .map(|&(i, j)| result[i][j].len() as i32)
        .sum();

    println!("part one result {}", result);

    Ok(())
}

fn part_two() -> Result<()> {
    let map = get_map_from_input()?;
    let n = map.len() as i32;
    let m = map
        .get(0)
        .context("could not get the first item in map")?
        .len() as i32;

    let mut result: Vec<Vec<i32>> = repeat(repeat(0).take(m as usize).collect())
        .take(n as usize)
        .collect();

    get_positions_with_value(9, &map)
        .iter()
        .for_each(|&(i, j)| result[i][j] = 1);

    let dxdy: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    (0..9).rev().for_each(|step| {
        get_positions_with_value(step, &map)
            .iter()
            .for_each(|&(i, j)| {
                let step_neighbours: Vec<(usize, usize)> = dxdy
                    .iter()
                    .map(|&(dx, dy)| (i as i32 + dx, j as i32 + dy))
                    .filter(|&(ni, nj)| 0 <= ni && ni < n && 0 <= nj && nj < m)
                    .map(|(ni, nj)| (ni as usize, nj as usize))
                    .filter(|&(ni, nj)| map[ni][nj] == step + 1)
                    .collect();
                step_neighbours.iter().for_each(|&(ni, nj)| {
                    result[i][j] += result[ni][nj];
                });
            })
    });

    let result: i32 = get_positions_with_value(0, &map)
        .iter()
        .map(|&(i, j)| result[i][j])
        .sum();

    println!("part two result {}", result);

    Ok(())
}

pub fn solve() -> Result<()> {
    part_two()
}
