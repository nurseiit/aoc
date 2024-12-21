use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};

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

fn all_dir_pad_solutions_for_nums(nums: &Vec<NumPadKey>) -> Vec<Vec<DirPadKey>> {
    fn paths_from_to(
        from: (usize, usize),
        to: (usize, usize),
        num_pad: &Vec<Vec<NumPadKey>>,
    ) -> HashSet<Vec<DirPadKey>> {
        let (n, m) = (num_pad.len(), num_pad[0].len());
        let mut paths = HashSet::new();

        let dist = manhattan_dist(from, to);
        let mut queue: VecDeque<(Vec<DirPadKey>, (usize, usize))> = VecDeque::new();
        let mut was: HashSet<Vec<DirPadKey>> = HashSet::new();

        queue.push_back((vec![], from));

        while let Some(top) = queue.pop_front() {
            let (mut dirs, position) = top;
            if dirs.len() == dist {
                if position == to {
                    dirs.push(DirPadKey::Enter);
                    paths.insert(dirs);
                }
                continue;
            }
            let (i, j) = position;
            vec![
                ((i.wrapping_sub(1), j), DirPadKey::Up),
                ((i + 1, j), DirPadKey::Down),
                ((i, j.wrapping_sub(1)), DirPadKey::Left),
                ((i, j + 1), DirPadKey::Right),
            ]
            .iter()
            .filter(|((ni, nj), _)| *ni < n && *nj < m && num_pad[*ni][*nj] != NumPadKey::Empty)
            .for_each(|(next_position, dir)| {
                let mut next_dirs = dirs.clone();
                next_dirs.push(*dir);

                if was.contains(&next_dirs) {
                    return;
                }

                queue.push_back((next_dirs.clone(), *next_position));
                was.insert(next_dirs);
            });
        }

        paths
    }

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
            paths_from_to(from, to, &num_pad)
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
    fn paths_from_to(
        from: (usize, usize),
        to: (usize, usize),
        dir_pad: &Vec<Vec<DirPadKey>>,
    ) -> HashSet<Vec<DirPadKey>> {
        let (n, m) = (dir_pad.len(), dir_pad[0].len());
        let mut paths = HashSet::new();

        let dist = manhattan_dist(from, to);
        let mut queue: VecDeque<(Vec<DirPadKey>, (usize, usize))> = VecDeque::new();
        let mut was: HashSet<Vec<DirPadKey>> = HashSet::new();

        queue.push_back((vec![], from));

        while let Some(top) = queue.pop_front() {
            let (mut dirs, position) = top;
            if dirs.len() == dist {
                if position == to {
                    dirs.push(DirPadKey::Enter);
                    paths.insert(dirs);
                }
                continue;
            }
            let (i, j) = position;
            vec![
                ((i.wrapping_sub(1), j), DirPadKey::Up),
                ((i + 1, j), DirPadKey::Down),
                ((i, j.wrapping_sub(1)), DirPadKey::Left),
                ((i, j + 1), DirPadKey::Right),
            ]
            .iter()
            .filter(|((ni, nj), _)| *ni < n && *nj < m && dir_pad[*ni][*nj] != DirPadKey::Empty)
            .for_each(|(next_position, dir)| {
                let mut next_dirs = dirs.clone();
                next_dirs.push(*dir);

                if was.contains(&next_dirs) {
                    return;
                }

                queue.push_back((next_dirs.clone(), *next_position));
                was.insert(next_dirs);
            });
        }

        paths
    }

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

    let set = dirs_from_enter
        .windows(2)
        .map(|window| {
            let from = get_position_in_dir_pad(window[0], &dir_pad);
            let to = get_position_in_dir_pad(window[1], &dir_pad);
            paths_from_to(from, to, &dir_pad)
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

pub fn solve() -> Result<()> {
    let input = read_input("input")?;

    let result: u32 = input
        .iter()
        .map(|num| {
            let all_num_path_dirs = all_dir_pad_solutions_for_nums(num);

            let min_for_num = all_num_path_dirs
                .iter()
                .flat_map(|num_path_dirs| all_dir_pad_solutions_for_dirs(num_path_dirs))
                .flat_map(|dir_path_dirs| all_dir_pad_solutions_for_dirs(&dir_path_dirs))
                .map(|result| result.len() as u32)
                .min()
                .expect("should have at least one solution");

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
