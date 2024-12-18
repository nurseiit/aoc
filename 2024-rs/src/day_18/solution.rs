use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::read_to_string,
    usize,
};

use anyhow::{Context, Result};

fn read_coordinates(file_name: &str) -> Result<Vec<(usize, usize)>> {
    let input =
        read_to_string(format!("./src/day_18/{}.txt", file_name)).context("could not read file")?;
    input
        .lines()
        .map(|line| {
            let coord = line
                .split(",")
                .map(|num| {
                    num.parse::<usize>()
                        .context(format!("could not format num '{}'", num,))
                })
                .collect::<Result<Vec<usize>>>()?;
            Ok((coord[1], coord[0]))
        })
        .collect()
}

fn shortest_path(
    corrupted_coords: &HashSet<(usize, usize)>,
    end_point: (usize, usize),
) -> Result<i32> {
    let start_point: (usize, usize) = (0, 0);

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut dist: HashMap<(usize, usize), i32> = HashMap::new();

    let dxdy: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    dist.insert(start_point, 0);
    queue.push_back(start_point);

    while let Some(top) = queue.pop_front() {
        let (i, j) = top;
        let cost = *dist.get(&top).expect("should have dist already");

        dxdy.iter()
            .map(|&(di, dj)| (i as i32 + di, j as i32 + dj))
            .filter(|&(ni, nj)| {
                ni >= 0 && ni <= end_point.0 as i32 && nj >= 0 && nj <= end_point.1 as i32
            })
            .for_each(|(ni, nj)| {
                let neighbour = (ni as usize, nj as usize);
                if dist.contains_key(&neighbour) || corrupted_coords.contains(&neighbour) {
                    return;
                }
                dist.insert(neighbour, cost + 1);
                queue.push_back(neighbour);
            });
    }

    dist.get(&end_point).context("no path found").copied()
}

fn part_one() -> Result<()> {
    let end_point: (usize, usize) = (70, 70);
    let input_coords = read_coordinates("input")?;
    let coords = input_coords[0..1024].to_vec();

    let corrupted_coords: HashSet<(usize, usize)> = HashSet::from_iter(coords.into_iter());

    let result = shortest_path(&corrupted_coords, end_point)?;

    println!("part one result: {}", result);

    Ok(())
}

fn part_two() -> Result<()> {
    let end_point: (usize, usize) = (70, 70);
    let input_coords = read_coordinates("input")?;
    let mut corrupted_coords: HashSet<(usize, usize)> = HashSet::new();

    if let Some(result) = (0..input_coords.len()).find(|&mid| {
        corrupted_coords.insert(input_coords[mid]);
        shortest_path(&corrupted_coords, end_point).is_err()
    }) {
        println!(
            "part two result: {:?}",
            (input_coords[result].1, input_coords[result].0)
        );
    } else {
        println!("part two: no answer");
    }

    Ok(())
}

pub fn solve() -> Result<()> {
    part_one()?;
    part_two()
}
