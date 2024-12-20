use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::read_to_string,
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

fn shortest_path(
    walls: &HashSet<(usize, usize)>,
    start: (usize, usize),
    finish: (usize, usize),
    dimensions: (usize, usize),
) -> u32 {
    let (n, m) = dimensions;

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut dist: HashMap<(usize, usize), u32> = HashMap::new();

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

    *dist.get(&finish).expect("no path found!")
}

pub fn solve() -> Result<()> {
    let maze = read_maze("input")?;

    let (n, m) = (maze.len(), maze[0].len());

    let mut start = Default::default();
    let mut finish = Default::default();

    let mut walls: HashSet<(usize, usize)> = HashSet::new();

    for i in 0..n {
        for j in 0..m {
            match maze[i][j] {
                MazeCell::Start => {
                    start = (i, j);
                }
                MazeCell::Finish => {
                    finish = (i, j);
                }
                MazeCell::Wall => {
                    walls.insert((i, j));
                }
                _ => {}
            };
        }
    }
    let original_shortest_path = shortest_path(&walls, start, finish, (n, m));
    let diff = 100;

    let part_one_result = walls
        .clone()
        .into_iter()
        .map(|(i, j)| {
            vec![(i + 1, j), (i, j + 1)]
                .into_iter()
                .filter(|&(ni, nj)| ni < n && nj < m)
                .filter(|neighbour| {
                    if walls.remove(neighbour) {
                        let current_path = shortest_path(&walls, start, finish, (n, m));
                        walls.insert(*neighbour);
                        return original_shortest_path - current_path >= diff;
                    }
                    false
                })
                .count()
        })
        .sum::<usize>();

    println!("part one result: {}", part_one_result);

    Ok(())
}
