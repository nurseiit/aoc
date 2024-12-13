use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
    iter::repeat,
};

use anyhow::{Context, Result};

fn read_table() -> Result<Vec<Vec<char>>> {
    let input = read_to_string("./src/day_12/input.txt").context("could not read file")?;
    Ok(input.lines().map(|line| line.chars().collect()).collect())
}

fn get_valid_neighbours((i, j): (usize, usize), (n, m): (usize, usize)) -> Vec<(usize, usize)> {
    let dxdy: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut valids = vec![];

    for (dx, dy) in dxdy {
        let ni = dx + i as i32;
        let nj = dy + j as i32;
        if ni < 0 || ni >= n as i32 || nj < 0 || nj >= m as i32 {
            continue;
        }
        valids.push((ni as usize, nj as usize));
    }

    valids
}

fn process_islands(visited_with_id: &mut Vec<Vec<i32>>, id: &mut i32, table: Vec<Vec<char>>) {
    for i in 0..table.len() {
        for j in 0..table[i].len() {
            if visited_with_id[i][j] != -1 {
                continue;
            }
            let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
            queue.push_back((i, j));
            while let Some(top) = queue.pop_front() {
                let (vi, vj) = top;
                visited_with_id[vi][vj] = *id;
                let neighbours: Vec<(usize, usize)> =
                    get_valid_neighbours((vi, vj), (table.len(), table[i].len()))
                        .into_iter()
                        .filter(|&(ni, nj)| {
                            table[vi][vj] == table[ni][nj] && visited_with_id[ni][nj] == -1
                        })
                        .collect();
                neighbours.into_iter().for_each(|(ni, nj)| {
                    queue.push_back((ni, nj));
                    visited_with_id[ni][nj] = *id;
                });
            }
            *id += 1;
        }
    }
}

fn part_one() -> Result<()> {
    let table = read_table()?;

    let n = table.len();
    let m = table
        .get(0)
        .context("could not get the first of table")?
        .len();

    let mut visited_with_id: Vec<Vec<i32>> = repeat(repeat(-1).take(m).collect()).take(n).collect();
    let mut id = 0;

    process_islands(&mut visited_with_id, &mut id, table);

    let mut area_by_id: HashMap<i32, i32> = HashMap::new();
    let mut perimeter_by_id: HashMap<i32, i32> = HashMap::new();

    for i in 0..n {
        for j in 0..m {
            let current_id = visited_with_id[i][j];
            *area_by_id.entry(current_id).or_insert(0) += 1;
            *perimeter_by_id.entry(current_id).or_insert(0) += get_valid_neighbours((i, j), (n, m))
                .into_iter()
                .filter(|&(ni, nj)| current_id != visited_with_id[ni][nj])
                .count() as i32;
            if i == 0 || i == n - 1 {
                *perimeter_by_id.entry(current_id).or_insert(0) += 1;
            }
            if j == 0 || j == m - 1 {
                *perimeter_by_id.entry(current_id).or_insert(0) += 1;
            }
        }
    }

    let result: i32 = (0..=id)
        .map(|current_id| {
            *area_by_id.get(&current_id).unwrap_or(&0)
                * *perimeter_by_id.get(&current_id).unwrap_or(&0)
        })
        .sum();

    println!("part one result {}", result);

    Ok(())
}

#[derive(Debug, Clone)]
struct Angle {
    /// 2d dxdy-s that should match relative to a given position
    hits: Vec<(i32, i32)>,
    /// 2d dxdy-s that should NOT match relative to a given position
    misses: Vec<(i32, i32)>,
}

fn position_matches_angle((i, j): (usize, usize), angle: Angle, table: &Vec<Vec<i32>>) -> bool {
    let n = table.len() as i32;
    let m = table[0].len() as i32;

    let current_id = table[i][j];

    angle.hits.into_iter().all(|(dx, dy)| {
        let ni = i as i32 + dx;
        let nj = j as i32 + dy;
        if ni < 0 || ni >= n || nj < 0 || nj >= m {
            return false;
        }
        current_id == table[ni as usize][nj as usize]
    }) && angle.misses.into_iter().all(|(dx, dy)| {
        let ni = i as i32 + dx;
        let nj = j as i32 + dy;
        if ni < 0 || ni >= n || nj < 0 || nj >= m {
            return true;
        }
        current_id != table[ni as usize][nj as usize]
    })
}

fn part_two() -> Result<()> {
    let table = read_table()?;

    let n = table.len();
    let m = table
        .get(0)
        .context("could not get the first of table")?
        .len();

    let mut visited_with_id: Vec<Vec<i32>> = repeat(repeat(-1).take(m).collect()).take(n).collect();
    let mut id = 0;

    process_islands(&mut visited_with_id, &mut id, table);

    let angles: Vec<Angle> = vec![
        // outer anglers
        // top left
        Angle {
            hits: vec![],
            misses: vec![(-1, 0), (0, -1)],
        },
        // top right
        Angle {
            hits: vec![],
            misses: vec![(-1, 0), (0, 1)],
        },
        // bottom right
        Angle {
            hits: vec![],
            misses: vec![(0, 1), (1, 0)],
        },
        // bottom left
        Angle {
            hits: vec![],
            misses: vec![(0, -1), (1, 0)],
        },
        // inner angles
        // top left
        Angle {
            hits: vec![(0, 1), (1, 0)],
            misses: vec![(1, 1)],
        },
        // top right
        Angle {
            hits: vec![(0, -1), (1, 0)],
            misses: vec![(1, -1)],
        },
        // bottom right
        Angle {
            hits: vec![(-1, 0), (0, -1)],
            misses: vec![(-1, -1)],
        },
        // bottom left
        Angle {
            hits: vec![(-1, 0), (0, 1)],
            misses: vec![(-1, 1)],
        },
    ];

    let mut area_by_id: HashMap<i32, i32> = HashMap::new();
    let mut sides_by_id: HashMap<i32, i32> = HashMap::new();

    for i in 0..n {
        for j in 0..m {
            let current_id = visited_with_id[i][j];
            *area_by_id.entry(current_id).or_insert(0) += 1;

            *sides_by_id.entry(current_id).or_insert(0) += angles
                .clone()
                .into_iter()
                .filter(|angle| position_matches_angle((i, j), angle.clone(), &visited_with_id))
                .count() as i32;
        }
    }

    let result: i32 = (0..=id)
        .map(|current_id| {
            *area_by_id.get(&current_id).unwrap_or(&0) * *sides_by_id.get(&current_id).unwrap_or(&0)
        })
        .sum();

    println!("part two result {}", result);

    Ok(())
}

pub fn solve() -> Result<()> {
    part_two()
}
