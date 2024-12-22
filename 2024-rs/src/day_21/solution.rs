use anyhow::{anyhow, Context, Ok, Result};
use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum NumPadKey {
    Enter,
    Number(u64),
    Empty,
}

fn read_input(file_name: &str) -> Result<Vec<Vec<NumPadKey>>> {
    let input =
        read_to_string(format!("./src/day_21/{}.txt", file_name)).context("could not read file")?;

    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    'A' => Ok(NumPadKey::Enter),
                    _ if char.is_ascii_digit() => Ok(NumPadKey::Number(
                        char.to_digit(10).expect("should be ascii digit").into(),
                    )),
                    _ => Err(anyhow!("unknown num pad key '{}'", char)),
                })
                .collect()
        })
        .collect()
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum DirPadKey {
    Up,
    Down,
    Left,
    Right,
    Enter,
    Empty,
}
const NUM_PAD: [[NumPadKey; 3]; 4] = [
    [
        NumPadKey::Number(7),
        NumPadKey::Number(8),
        NumPadKey::Number(9),
    ],
    [
        NumPadKey::Number(4),
        NumPadKey::Number(5),
        NumPadKey::Number(6),
    ],
    [
        NumPadKey::Number(1),
        NumPadKey::Number(2),
        NumPadKey::Number(3),
    ],
    [NumPadKey::Empty, NumPadKey::Number(0), NumPadKey::Enter],
];

const DIR_PAD: [[DirPadKey; 3]; 2] = [
    [DirPadKey::Empty, DirPadKey::Up, DirPadKey::Enter],
    [DirPadKey::Left, DirPadKey::Down, DirPadKey::Right],
];

fn shortest_num_pad_paths(from: NumPadKey, to: NumPadKey) -> Vec<Vec<DirPadKey>> {
    let mut start = (0, 0);
    let mut end = (0, 0);

    for i in 0..NUM_PAD.len() {
        for j in 0..NUM_PAD[i].len() {
            if NUM_PAD[i][j] == from {
                start = (i, j);
            }
            if NUM_PAD[i][j] == to {
                end = (i, j);
            }
        }
    }

    let is_up = end.0 < start.0;
    let is_left = end.1 < start.1;

    let dy = if is_up {
        start.0 - end.0
    } else {
        end.0 - start.0
    };
    let dx = if is_left {
        start.1 - end.1
    } else {
        end.1 - start.1
    };

    let mut result = vec![];

    // horizontal
    let mut horizontal = vec![];
    let mut is_horizontal_good = true;
    if dx > 0 {
        for x in 1..=dx {
            let (ni, nj) = (start.0, if is_left { start.1 - x } else { start.1 + x });
            if NUM_PAD[ni][nj] == NumPadKey::Empty {
                is_horizontal_good = false;
            }
            horizontal.push(if is_left {
                DirPadKey::Left
            } else {
                DirPadKey::Right
            });
        }
    }
    if dy > 0 {
        for y in 1..=dy {
            let (ni, nj) = (if is_up { start.0 - y } else { start.0 + y }, end.1);
            if NUM_PAD[ni][nj] == NumPadKey::Empty {
                is_horizontal_good = false;
            }
            horizontal.push(if is_up {
                DirPadKey::Up
            } else {
                DirPadKey::Down
            });
        }
    }
    // vertical
    let mut vertical = vec![];
    let mut is_vertical_good = true;
    if dy > 0 {
        for y in 1..=dy {
            let (ni, nj) = (if is_up { start.0 - y } else { start.0 + y }, start.1);
            if NUM_PAD[ni][nj] == NumPadKey::Empty {
                is_vertical_good = false;
            }
            vertical.push(if is_up {
                DirPadKey::Up
            } else {
                DirPadKey::Down
            });
        }
    }
    if dx > 0 {
        for x in 1..=dx {
            let (ni, nj) = (end.0, if is_left { start.1 - x } else { start.1 + x });
            if NUM_PAD[ni][nj] == NumPadKey::Empty {
                is_vertical_good = false;
            }
            vertical.push(if is_left {
                DirPadKey::Left
            } else {
                DirPadKey::Right
            });
        }
    }

    if is_horizontal_good && (dx > 0 && dy > 0) {
        horizontal.push(DirPadKey::Enter);
        result.push(horizontal);
    }
    if is_vertical_good {
        vertical.push(DirPadKey::Enter);
        result.push(vertical);
    }

    result
}

fn shortest_dir_pad_paths(from: DirPadKey, to: DirPadKey) -> Vec<Vec<DirPadKey>> {
    match from {
        DirPadKey::Up => match to {
            DirPadKey::Up => vec![vec![DirPadKey::Enter]],
            DirPadKey::Down => vec![vec![DirPadKey::Down, DirPadKey::Enter]],
            DirPadKey::Left => vec![vec![DirPadKey::Down, DirPadKey::Left, DirPadKey::Enter]],
            DirPadKey::Right => vec![
                vec![DirPadKey::Down, DirPadKey::Right, DirPadKey::Enter],
                vec![DirPadKey::Right, DirPadKey::Down, DirPadKey::Enter],
            ],
            DirPadKey::Enter => vec![vec![DirPadKey::Right, DirPadKey::Enter]],
            _ => vec![],
        },
        DirPadKey::Down => match to {
            DirPadKey::Up => vec![vec![DirPadKey::Up, DirPadKey::Enter]],
            DirPadKey::Down => vec![vec![DirPadKey::Enter]],
            DirPadKey::Left => vec![vec![DirPadKey::Left, DirPadKey::Enter]],
            DirPadKey::Right => vec![vec![DirPadKey::Right, DirPadKey::Enter]],
            DirPadKey::Enter => vec![
                vec![DirPadKey::Up, DirPadKey::Right, DirPadKey::Enter],
                vec![DirPadKey::Right, DirPadKey::Up, DirPadKey::Enter],
            ],
            _ => vec![],
        },
        DirPadKey::Left => match to {
            DirPadKey::Up => vec![vec![DirPadKey::Right, DirPadKey::Up, DirPadKey::Enter]],
            DirPadKey::Down => vec![vec![DirPadKey::Right, DirPadKey::Enter]],
            DirPadKey::Left => vec![vec![DirPadKey::Enter]],
            DirPadKey::Right => vec![vec![DirPadKey::Right, DirPadKey::Right, DirPadKey::Enter]],
            DirPadKey::Enter => vec![
                vec![
                    DirPadKey::Right,
                    DirPadKey::Up,
                    DirPadKey::Right,
                    DirPadKey::Enter,
                ],
                vec![
                    DirPadKey::Right,
                    DirPadKey::Right,
                    DirPadKey::Up,
                    DirPadKey::Enter,
                ],
            ],
            _ => vec![],
        },
        DirPadKey::Right => match to {
            DirPadKey::Up => vec![
                vec![DirPadKey::Left, DirPadKey::Up, DirPadKey::Enter],
                vec![DirPadKey::Up, DirPadKey::Left, DirPadKey::Enter],
            ],
            DirPadKey::Down => vec![vec![DirPadKey::Left, DirPadKey::Enter]],
            DirPadKey::Left => vec![vec![DirPadKey::Left, DirPadKey::Left, DirPadKey::Enter]],
            DirPadKey::Right => vec![vec![DirPadKey::Enter]],
            DirPadKey::Enter => vec![vec![DirPadKey::Up, DirPadKey::Enter]],
            _ => vec![],
        },
        DirPadKey::Enter => match to {
            DirPadKey::Up => vec![vec![DirPadKey::Left, DirPadKey::Enter]],
            DirPadKey::Down => vec![
                vec![DirPadKey::Left, DirPadKey::Down, DirPadKey::Enter],
                vec![DirPadKey::Down, DirPadKey::Left, DirPadKey::Enter],
            ],
            DirPadKey::Left => vec![
                vec![
                    DirPadKey::Down,
                    DirPadKey::Left,
                    DirPadKey::Left,
                    DirPadKey::Enter,
                ],
                vec![
                    DirPadKey::Left,
                    DirPadKey::Down,
                    DirPadKey::Left,
                    DirPadKey::Enter,
                ],
            ],
            DirPadKey::Right => vec![vec![DirPadKey::Down, DirPadKey::Enter]],
            DirPadKey::Enter => vec![vec![DirPadKey::Enter]],
            _ => vec![],
        },
        _ => vec![],
    }
}

fn all_dir_pad_solutions_for_nums(nums: &Vec<NumPadKey>) -> Vec<Vec<Vec<DirPadKey>>> {
    let nums_from_enter = vec![vec![NumPadKey::Enter], nums.clone()].concat();

    nums_from_enter
        .windows(2)
        .map(|window| shortest_num_pad_paths(window[0], window[1]))
        .collect()
}

fn count_paths(
    dir: &Vec<DirPadKey>,
    level: usize,
    cache: &mut HashMap<(Vec<DirPadKey>, usize), u64>,
) -> u64 {
    if level == 0 {
        return dir.len() as u64;
    }

    if let Some(cached) = cache.get(&(dir.clone(), level)) {
        return *cached;
    }

    let dirs_from_enter = vec![vec![DirPadKey::Enter], dir.clone()].concat();

    let computed = dirs_from_enter
        .windows(2)
        .map(|window| {
            shortest_dir_pad_paths(window[0], window[1])
                .into_iter()
                .map(|path| count_paths(&path, level - 1, cache))
                .min()
                .expect("should have it")
        })
        .sum::<u64>();

    cache.insert((dir.clone(), level), computed);

    return computed;
}

pub fn solve() -> Result<()> {
    let levels_count = 25;
    let input = read_input("input")?;

    let mut cache: HashMap<(Vec<DirPadKey>, usize), u64> = HashMap::new();

    let result: u64 = input
        .iter()
        .map(|num| {
            let min_for_num = all_dir_pad_solutions_for_nums(num)
                .iter()
                .map(|dirs| {
                    dirs.iter()
                        .map(|dir| count_paths(dir, levels_count, &mut cache))
                        .min()
                        .unwrap()
                })
                .sum::<u64>();

            let int: u64 = num.iter().fold(0, |acc, x| match *x {
                NumPadKey::Number(digit) => acc * 10 + digit,
                _ => acc,
            });

            int * min_for_num
        })
        .sum();

    println!("result: {}", result);

    Ok(())
}
