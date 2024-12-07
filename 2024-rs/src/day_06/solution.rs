use std::fs::read_to_string;

use anyhow::{Context, Error, Result};

fn read_room_from_file(file_path: &str) -> Result<Vec<Vec<char>>, Error> {
    let input = read_to_string(file_path).context("could not read file")?;
    let room = input.lines().map(|line| line.chars().collect()).collect();
    Ok(room)
}

fn get_next_direction(direction: char) -> Result<char, Error> {
    match direction {
        '^' => Ok('>'),
        '>' => Ok('V'),
        'V' => Ok('<'),
        '<' => Ok('^'),
        _ => Err(Error::msg("incorrect direction")),
    }
}

fn get_delta_from_direction(direction: char) -> Result<(i32, i32), Error> {
    match direction {
        '^' => Ok((-1, 0)),
        '>' => Ok((0, 1)),
        'V' => Ok((1, 0)),
        '<' => Ok((0, -1)),
        _ => Err(Error::msg("incorrect direction")),
    }
}

fn is_guard(cell: char) -> bool {
    let directions = "^>V<";
    directions.contains(cell)
}

fn is_obstacle(cell: char) -> bool {
    cell == '#'
}

fn is_footprint(cell: char) -> bool {
    cell == 'X'
}

fn part_one() -> Result<(), Error> {
    let room = read_room_from_file("./src/day_06/input.txt")?;

    let n = room.len() as i32;
    let m = room.get(0).context("empty room")?.len() as i32;

    let mut position: (i32, i32) = (-1, -1);

    for i in 0..n {
        for j in 0..m {
            if is_guard(room[i as usize][j as usize]) {
                position = (i, j);
                break;
            }
        }
        if position != (-1, -1) {
            break;
        }
    }

    if position == (-1, -1) {
        return Err(Error::msg("no guard found"));
    }

    let mut footprints = room.clone();

    fn is_within(it: i32, from: i32, to: i32) -> bool {
        from <= it && it < to
    }

    loop {
        let (i, j) = position;
        let direction = footprints[i as usize][j as usize];
        let (di, dj) = get_delta_from_direction(direction)?;

        let (ni, nj) = (i + di, j + dj);

        if !is_within(ni, 0, n) || !is_within(nj, 0, m) {
            break;
        }
        let next_cell = footprints[ni as usize][nj as usize];
        if is_obstacle(next_cell) {
            footprints[i as usize][j as usize] = get_next_direction(direction)?;
        } else {
            footprints[i as usize][j as usize] = 'X';
            footprints[ni as usize][nj as usize] = direction;
            position = (ni, nj);
        }
    }

    let result: i32 = footprints
        .into_iter()
        .map(|line| {
            line.into_iter()
                .filter(|cell| is_guard(*cell) || is_footprint(*cell))
                .collect::<Vec<_>>()
                .len() as i32
        })
        .sum();

    println!("part one result {}", result);

    Ok(())
}

pub fn solve() -> Result<(), Error> {
    part_one()
}
