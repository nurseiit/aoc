use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::read_to_string,
    u32,
};

use anyhow::{anyhow, Context, Ok, Result};

#[derive(Debug, Hash, PartialEq, Eq)]
enum MazeCell {
    Empty,
    Wall,
    Start,
    Finish,
}

fn read_maze(file_name: &str) -> Result<Vec<Vec<MazeCell>>> {
    let input =
        read_to_string(format!("./src/day_20/{}.txt", file_name)).context("could not read file")?;

    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|cell| match cell {
                    '.' => Ok(MazeCell::Empty),
                    '#' => Ok(MazeCell::Wall),
                    'S' => Ok(MazeCell::Start),
                    'E' => Ok(MazeCell::Finish),
                    _ => Err(anyhow!("unknown maze cell '{}'", cell)),
                })
                .collect()
        })
        .collect()
}

fn get_neighbours((i, j): (usize, usize), (n, m): (usize, usize)) -> Vec<(usize, usize)> {
    vec![
        (i, j.wrapping_sub(1)),
        (i, j + 1),
        (i.wrapping_sub(1), j),
        (i + 1, j),
    ]
    .into_iter()
    .filter(|&(ni, nj)| ni < n && nj < m)
    .collect()
}

fn shortest_path_distances(
    walls: &HashSet<(usize, usize)>,
    start: (usize, usize),
    finish: (usize, usize),
    dimensions: (usize, usize),
) -> HashMap<(usize, usize), u32> {
    let (n, m) = dimensions;

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut dist = HashMap::new();

    queue.push_back(start);
    dist.insert(start, 0);

    while let Some(top) = queue.pop_front() {
        if top == finish {
            break;
        }

        get_neighbours(top, (n, m))
            .into_iter()
            .filter(|neighbour| !walls.contains(neighbour))
            .for_each(|(ni, nj)| {
                if dist.contains_key(&(ni, nj)) {
                    return;
                }
                let cost = dist.get(&top).expect("should have distance once here");
                dist.insert((ni, nj), *cost + 1);
                queue.push_back((ni, nj));
            });
    }

    dist
}

fn manhattan_dist(from: (usize, usize), to: (usize, usize)) -> u32 {
    let x = from.0.max(to.0) - from.0.min(to.0);
    let y = from.1.max(to.1) - from.1.min(to.1);
    (x + y) as u32
}

pub fn solve() -> Result<()> {
    let max_cheat = 20;
    let diff = 100;
    let maze = read_maze("input")?;

    let (n, m) = (maze.len(), maze[0].len());

    let mut start = Default::default();
    let mut finish = Default::default();

    let mut walls: HashSet<(usize, usize)> = HashSet::new();
    let mut empties = vec![];

    for i in 0..n {
        for j in 0..m {
            match maze[i][j] {
                MazeCell::Start => {
                    start = (i, j);
                    empties.push((i, j));
                }
                MazeCell::Finish => {
                    finish = (i, j);
                    empties.push((i, j));
                }
                MazeCell::Wall => {
                    walls.insert((i, j));
                }
                MazeCell::Empty => {
                    empties.push((i, j));
                }
            };
        }
    }
    let dist_from_start = shortest_path_distances(&walls, start, finish, (n, m));
    let mut result = 0;

    for i in 0..empties.len() {
        for j in 0..empties.len() {
            if i == j {
                continue;
            }
            let from = empties[i];
            let to = empties[j];
            let dist_to_from = *dist_from_start.get(&from).unwrap();
            let dist_to_to = *dist_from_start.get(&to).unwrap();
            let cheat = manhattan_dist(from, to);

            if cheat > max_cheat {
                continue;
            }

            if dist_to_from + cheat >= dist_to_to {
                continue;
            }

            if dist_to_from + cheat + diff > dist_to_to {
                continue;
            }

            result += 1;
        }
    }

    println!("result: {}", result);

    Ok(())
}
