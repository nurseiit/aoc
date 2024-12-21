use std::{collections::HashSet, fs::read_to_string, iter::repeat};

use anyhow::{anyhow, Context, Ok, Result};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum NumPadKey {
    Enter,
    Number(u32),
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
                        char.to_digit(10).expect("should be ascii digit"),
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

fn get_num_pad_matrix() -> Vec<Vec<NumPadKey>> {
    vec![
        vec![
            NumPadKey::Number(7),
            NumPadKey::Number(8),
            NumPadKey::Number(9),
        ],
        vec![
            NumPadKey::Number(4),
            NumPadKey::Number(5),
            NumPadKey::Number(6),
        ],
        vec![
            NumPadKey::Number(1),
            NumPadKey::Number(2),
            NumPadKey::Number(3),
        ],
        vec![NumPadKey::Empty, NumPadKey::Number(0), NumPadKey::Enter],
    ]
}

fn get_dir_pad_matrix() -> Vec<Vec<DirPadKey>> {
    vec![
        vec![DirPadKey::Empty, DirPadKey::Up, DirPadKey::Enter],
        vec![DirPadKey::Left, DirPadKey::Down, DirPadKey::Right],
    ]
}

fn manhattan_dist(from: (usize, usize), to: (usize, usize)) -> usize {
    let x = from.0.max(to.0) - from.0.min(to.0);
    let y = from.1.max(to.1) - from.1.min(to.1);
    x + y
}

fn paths_from_to(from: (usize, usize), to: (usize, usize)) -> HashSet<Vec<DirPadKey>> {
    if from == to {
        return HashSet::from_iter(vec![vec![DirPadKey::Enter]].into_iter());
    }
    let mut result = HashSet::new();
    if from.0 == to.0 {
        let dir = if from.1 < to.1 {
            DirPadKey::Right
        } else {
            DirPadKey::Left
        };
        let dist = from.1.max(to.1) - from.1.min(to.1);
        let mut dirs: Vec<DirPadKey> = repeat(dir).take(dist).collect();
        dirs.push(DirPadKey::Enter);
        result.insert(dirs);
    }
    if from.1 == to.1 {
        let dir = if from.0 < to.0 {
            DirPadKey::Down
        } else {
            DirPadKey::Up
        };
        let dist = from.0.max(to.0) - from.0.min(to.0);
        let mut dirs: Vec<DirPadKey> = repeat(dir).take(dist).collect();
        dirs.push(DirPadKey::Enter);
        result.insert(dirs);
    }

    if from.0 != to.0 && from.1 != to.1 {
        {
            let left_or_right_dir = if from.1 < to.1 {
                DirPadKey::Right
            } else {
                DirPadKey::Left
            };
            let left_to_right = from.1.max(to.1) - from.1.min(to.1);
            let mut dirs: Vec<DirPadKey> = repeat(left_or_right_dir).take(left_to_right).collect();

            let up_or_down_dir = if from.0 < to.0 {
                DirPadKey::Down
            } else {
                DirPadKey::Up
            };
            let up_to_down = from.0.max(to.0) - from.0.min(to.0);
            (0..up_to_down)
                .into_iter()
                .for_each(|_| dirs.push(up_or_down_dir));
            dirs.push(DirPadKey::Enter);
            result.insert(dirs);
        }
        {
            let up_or_down_dir = if from.0 < to.0 {
                DirPadKey::Down
            } else {
                DirPadKey::Up
            };
            let up_to_down = from.0.max(to.0) - from.0.min(to.0);
            let mut dirs: Vec<DirPadKey> = repeat(up_or_down_dir).take(up_to_down).collect();

            let left_or_right_dir = if from.1 < to.1 {
                DirPadKey::Right
            } else {
                DirPadKey::Left
            };
            let left_to_right = from.1.max(to.1) - from.1.min(to.1);

            (0..left_to_right)
                .into_iter()
                .for_each(|_| dirs.push(left_or_right_dir));
            dirs.push(DirPadKey::Enter);
            result.insert(dirs);
        }
    }
    result
}

fn all_dir_pad_solutions_for_nums(nums: &Vec<NumPadKey>) -> Vec<Vec<DirPadKey>> {
    fn get_position_in_num_pad(num: NumPadKey, num_pad: &Vec<Vec<NumPadKey>>) -> (usize, usize) {
        for i in 0..num_pad.len() {
            for j in 0..num_pad[i].len() {
                if num == num_pad[i][j] {
                    return (i, j);
                }
            }
        }
        (0, 0)
    }

    let num_pad = get_num_pad_matrix();

    let mut nums_from_enter = vec![NumPadKey::Enter];

    for num in nums {
        nums_from_enter.push(*num);
    }

    let set = nums_from_enter
        .windows(2)
        .map(|window| {
            let from = get_position_in_num_pad(window[0], &num_pad);
            let to = get_position_in_num_pad(window[1], &num_pad);
            paths_from_to(from, to)
                .into_iter()
                .filter(|path| {
                    let (mut i, mut j) = from;
                    let mut is_bad = false;
                    path.iter().for_each(|dir| {
                        match *dir {
                            DirPadKey::Up => {
                                i = i.wrapping_sub(1);
                            }
                            DirPadKey::Down => {
                                i += 1;
                            }
                            DirPadKey::Left => {
                                j = j.wrapping_sub(1);
                            }
                            DirPadKey::Right => {
                                j += 1;
                            }
                            _ => {}
                        };
                        if num_pad[i][j] == NumPadKey::Empty {
                            is_bad = true;
                        }
                    });
                    !is_bad
                })
                .collect()
        })
        .fold(HashSet::new(), |acc, cur| {
            if acc.is_empty() {
                return cur;
            }
            let mut next = HashSet::new();
            acc.iter().for_each(|prev_path| {
                cur.iter().for_each(|curr_path| {
                    let mut item = vec![];
                    for path in prev_path {
                        item.push(*path);
                    }
                    for path in curr_path {
                        item.push(*path);
                    }
                    next.insert(item);
                });
            });
            next
        });

    set.into_iter().collect()
}

fn all_dir_pad_solutions_for_dirs(dirs: &Vec<DirPadKey>) -> Vec<Vec<DirPadKey>> {
    fn get_position_in_dir_pad(dir: DirPadKey, dir_pad: &Vec<Vec<DirPadKey>>) -> (usize, usize) {
        for i in 0..dir_pad.len() {
            for j in 0..dir_pad[i].len() {
                if dir == dir_pad[i][j] {
                    return (i, j);
                }
            }
        }
        (0, 0)
    }

    let dir_pad = get_dir_pad_matrix();

    let mut dirs_from_enter = vec![DirPadKey::Enter];

    for dir in dirs {
        dirs_from_enter.push(*dir);
    }

    let dirs = dirs_from_enter
        .windows(2)
        .map(|window| {
            let from = get_position_in_dir_pad(window[0], &dir_pad);
            let to = get_position_in_dir_pad(window[1], &dir_pad);
            paths_from_to(from, to)
                .into_iter()
                .filter(|path| {
                    let (mut i, mut j) = from;
                    let mut is_bad = false;
                    path.iter().for_each(|dir| {
                        match *dir {
                            DirPadKey::Up => {
                                i = i.wrapping_sub(1);
                            }
                            DirPadKey::Down => {
                                i += 1;
                            }
                            DirPadKey::Left => {
                                j = j.wrapping_sub(1);
                            }
                            DirPadKey::Right => {
                                j += 1;
                            }
                            _ => {}
                        };
                        if dir_pad[i][j] == DirPadKey::Empty {
                            is_bad = true;
                        }
                    });
                    !is_bad
                })
                .collect()
        })
        .fold(HashSet::new(), |acc, cur| {
            if acc.is_empty() {
                return cur;
            }
            let mut next = HashSet::new();
            acc.iter().for_each(|prev_path| {
                cur.iter().for_each(|curr_path| {
                    let mut item = vec![];
                    for path in prev_path {
                        item.push(*path);
                    }
                    for path in curr_path {
                        item.push(*path);
                    }
                    next.insert(item);
                });
            });
            next
        });

    dirs.into_iter().collect()
}

pub fn solve() -> Result<()> {
    let levels_count = 2;
    let input = read_input("input")?;

    let result: u32 = input
        .iter()
        .map(|num| {
            let all_num_path_dirs = all_dir_pad_solutions_for_nums(num);

            println!("solving for {:?}", num);

            let mut current_dirs = all_num_path_dirs;

            for level in 0..levels_count {
                let min_for_level = current_dirs
                    .iter()
                    .map(|result| result.len() as u32)
                    .min()
                    .expect("should have at least one solution");

                println!(
                    "level {} with len {} and answer = {}...",
                    level,
                    current_dirs.len(),
                    min_for_level
                );

                current_dirs = current_dirs
                    .iter()
                    .flat_map(|dir_path_dirs| all_dir_pad_solutions_for_dirs(&dir_path_dirs))
                    .collect();
            }

            let min_for_num = current_dirs
                .iter()
                .map(|result| result.len() as u32)
                .min()
                .expect("should have at least one solution");

            println!("len = {}, answer = {}", current_dirs.len(), min_for_num);

            let int: u32 = num.iter().fold(0, |acc, x| match *x {
                NumPadKey::Number(digit) => acc * 10 + digit,
                _ => acc,
            });

            int * min_for_num
        })
        .sum();

    println!("part one result: {}", result);

    Ok(())
}
