use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

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

fn is_within(it: i32, from: i32, to: i32) -> bool {
    from <= it && it < to
}

fn get_original_footprint_indexes(
    mut footprints: Vec<Vec<char>>,
    mut position: (i32, i32),
    n: i32,
    m: i32,
) -> Result<Vec<(usize, usize)>, Error> {
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

    Ok(footprints
        .into_iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.into_iter()
                .enumerate()
                .filter(|(_, cell)| is_guard(*cell) || is_footprint(*cell))
                .map(|(j, _)| (i, j))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect())
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

    println!(
        "part one result {}",
        get_original_footprint_indexes(room.clone(), position, n, m)?.len()
    );

    Ok(())
}

fn part_two() -> Result<(), Error> {
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

    fn is_infinite_loop(
        footprints: &mut Vec<Vec<char>>,
        mut position: (i32, i32),
        n: i32,
        m: i32,
    ) -> Result<bool, Error> {
        let mut all_changes: HashMap<(i32, i32), char> = HashMap::new();
        let mut history: HashSet<(i32, i32, char)> = HashSet::new();
        let mut is_infinite = false;
        loop {
            let (i, j) = position;
            let direction = footprints[i as usize][j as usize];
            let (di, dj) = get_delta_from_direction(direction)?;

            if history.contains(&(i, j, direction)) {
                is_infinite = true;
                break;
            }

            history.insert((i, j, direction));

            let (ni, nj) = (i + di, j + dj);

            if !is_within(ni, 0, n) || !is_within(nj, 0, m) {
                break;
            }
            let next_cell = footprints[ni as usize][nj as usize];
            if is_obstacle(next_cell) {
                if !all_changes.contains_key(&(i, j)) {
                    all_changes.insert((i, j), footprints[i as usize][j as usize]);
                }
                footprints[i as usize][j as usize] = get_next_direction(direction)?;
            } else {
                if !all_changes.contains_key(&(i, j)) {
                    all_changes.insert((i, j), footprints[i as usize][j as usize]);
                }
                if !all_changes.contains_key(&(ni, nj)) {
                    all_changes.insert((ni, nj), footprints[ni as usize][nj as usize]);
                }
                footprints[i as usize][j as usize] = 'X';
                footprints[ni as usize][nj as usize] = direction;
                position = (ni, nj);
            }
        }
        all_changes
            .into_iter()
            .for_each(|((i, j), cell)| footprints[i as usize][j as usize] = cell);
        Ok(is_infinite)
    }

    let mut result = 0;
    let mut footprints = room.clone();

    let indexes = get_original_footprint_indexes(room.clone(), position, n, m)?;

    for (id, (i, j)) in indexes.into_iter().enumerate() {
        let cell = room[i][j];
        if is_guard(cell) || is_obstacle(cell) {
            continue;
        }
        footprints[i][j] = '#';
        if is_infinite_loop(&mut footprints, position, n, m)? {
            result += 1;
        }
        footprints[i][j] = cell;
    }

    println!("part two result {}", result);

    Ok(())
}

pub fn solve() -> Result<(), Error> {
    part_two()
}
