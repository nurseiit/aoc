use std::{fs::read_to_string, iter::repeat};

use anyhow::{Context, Error, Result};

fn read_board(file_path: &str) -> Result<Vec<Vec<char>>> {
    let data = read_to_string(file_path).context("could not read file")?;
    let board = data
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    Ok(board)
}

fn part_one() -> Result<(), Error> {
    let board = read_board("./src/day_04/input.txt")?;

    let n = board.len();
    let m = board[0].len();

    let letters: Vec<char> = "XMAS".chars().collect();

    let mut result = 0;

    let dxdy: Vec<i32> = vec![-1, 0, 1];

    let mut new_board: Vec<Vec<char>> = repeat(repeat('.').take(m).collect()).take(n).collect();

    for ui in 0..n {
        for uj in 0..m {
            dxdy.iter().for_each(|dx| {
                dxdy.iter().for_each(|dy| {
                    let mut i = ui;
                    let mut j = uj;
                    let mut ok = true;
                    let mut history = vec![(i, j)];
                    for l in 0..letters.len() {
                        if board[i][j] != letters[l] {
                            ok = false;
                            break;
                        }
                        if l + 1 == letters.len() {
                            break;
                        }
                        let ni = i as i32 + *dx;
                        let nj = j as i32 + *dy;

                        if ni < 0 || ni >= n as i32 || nj < 0 || nj >= m as i32 {
                            ok = false;
                            break;
                        }
                        if ni == i as i32 && nj == j as i32 {
                            ok = false;
                            break;
                        }
                        i = ni as usize;
                        j = nj as usize;
                        history.push((i, j));
                    }
                    if ok {
                        result += 1;
                        history.iter().for_each(|(ni, nj)| {
                            new_board[*ni][*nj] = board[*ni][*nj];
                        });
                    }
                })
            });
        }
    }

    board
        .iter()
        .for_each(|line| println!("{}", String::from_iter(line.iter())));

    println!("\n");

    new_board
        .iter()
        .for_each(|line| println!("{}", String::from_iter(line.iter())));

    println!("part one result {}", result);

    Ok(())
}

pub fn solve() -> Result<(), Error> {
    part_one()
}
