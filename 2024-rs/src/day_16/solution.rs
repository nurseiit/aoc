use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fs::read_to_string,
    i32,
};

use anyhow::{Context, Error, Result};

#[derive(Debug, PartialEq, Eq)]
enum MazeCell {
    Wall,
    Empty,
    EndPoint,
    StartPoint,
    Path,
}

fn read_maze(file_name: &str) -> Result<Vec<Vec<MazeCell>>> {
    let input =
        read_to_string(format!("./src/day_16/{}.txt", file_name)).context("could not read file")?;

    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|cell| match cell {
                    '#' => Ok(MazeCell::Wall),
                    '.' => Ok(MazeCell::Empty),
                    'E' => Ok(MazeCell::EndPoint),
                    'S' => Ok(MazeCell::StartPoint),
                    _ => Err(Error::msg(format!("unknown maze cell '{}'", cell))),
                })
                .collect()
        })
        .collect()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct StepState {
    position: (usize, usize),
    direction: Direction,
    cost: i32,
    positions_set: HashSet<(usize, usize)>,
}

impl Ord for StepState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for StepState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn solve() -> Result<()> {
    let maze = read_maze("input")?;

    let n = maze.len();
    let m = maze[0].len();

    let mut start_point: (usize, usize) = Default::default();
    let mut end_point: (usize, usize) = Default::default();

    for i in 0..n {
        for j in 0..m {
            match maze[i][j] {
                MazeCell::StartPoint => start_point = (i, j),
                MazeCell::EndPoint => end_point = (i, j),
                _ => {}
            };
        }
    }

    let start_direction = Direction::East;

    let mut dist: HashMap<((usize, usize), Direction), i32> = HashMap::new();
    let mut heap: BinaryHeap<StepState> = BinaryHeap::new();

    let mut all_positions: HashSet<(usize, usize)> = HashSet::new();

    dist.insert((start_point, start_direction), 0);
    heap.push(StepState {
        position: start_point,
        direction: start_direction,
        cost: 0,
        positions_set: HashSet::from_iter(vec![start_point].into_iter()),
    });

    while let Some(StepState {
        position,
        direction,
        cost,
        positions_set,
    }) = heap.pop()
    {
        if position == end_point {
            let all_directions = vec![
                Direction::North,
                Direction::South,
                Direction::West,
                Direction::East,
            ];

            let min = all_directions
                .into_iter()
                .map(|direction| dist.get(&(end_point, direction)).unwrap_or(&i32::MAX))
                .min();

            if cost == *min.unwrap_or(&i32::MAX) {
                positions_set.into_iter().for_each(|item| {
                    all_positions.insert(item);
                });
            }
            continue;
        }

        if cost > *dist.get(&(position, direction)).unwrap_or(&i32::MAX) {
            continue;
        }

        // rotate
        let rotate_directions: Vec<Direction> = match direction {
            Direction::North | Direction::South => vec![Direction::East, Direction::West],
            Direction::East | Direction::West => vec![Direction::North, Direction::South],
        };

        rotate_directions.into_iter().for_each(|rotate_direction| {
            if cost + 1000 <= *dist.get(&(position, rotate_direction)).unwrap_or(&i32::MAX) {
                heap.push(StepState {
                    position,
                    direction: rotate_direction,
                    cost: cost + 1000,
                    positions_set: positions_set.clone(),
                });
                dist.insert((position, rotate_direction), cost + 1000);
            }
        });

        let (i, j) = position;
        let (ni, nj): (usize, usize) = match direction {
            Direction::North => (i - 1, j),
            Direction::South => (i + 1, j),
            Direction::East => (i, j + 1),
            Direction::West => (i, j - 1),
        };

        if ni >= n || nj >= m || maze[ni][nj] == MazeCell::Wall {
            continue;
        }

        let mut next_positions_set = positions_set.clone();
        next_positions_set.insert((ni, nj));

        let next_state = StepState {
            position: (ni, nj),
            direction,
            cost: cost + 1,
            positions_set: next_positions_set,
        };

        if next_state.cost
            <= *dist
                .get(&(next_state.position, direction))
                .unwrap_or(&i32::MAX)
        {
            heap.push(next_state.clone());
            dist.insert((next_state.position, direction), cost + 1);
        }
    }

    let all_directions = vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];

    let min = all_directions
        .into_iter()
        .map(|direction| dist.get(&(end_point, direction)).unwrap_or(&i32::MAX))
        .min();

    if let Some(&result) = min {
        println!("part one result {}", result);
    } else {
        println!("no path found to {:?}", end_point);
    }

    println!("part two result {}", all_positions.len());

    Ok(())
}
