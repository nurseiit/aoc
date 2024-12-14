use std::fs::read_to_string;

use anyhow::{Context, Result};
use regex::Regex;

#[derive(Debug, Clone)]
struct Game {
    buttons: Vec<(i64, i64)>,
    prize: (i64, i64),
}

fn read_games_input() -> Result<Vec<Game>> {
    let input = read_to_string("./src/day_13/input.txt").context("could not read file")?;

    let button_re = Regex::new(r"Button (A|B)\: X\+(?<x>\d+)\, Y\+(?<y>\d+)")?;
    let prize_re = Regex::new(r"Prize\: X\=(?<x>\d+)\, Y\=(?<y>\d+)")?;

    input
        .split("\n\n")
        .map(|game_input| {
            let buttons = button_re
                .captures_iter(game_input)
                .map(|capture| {
                    let x: i64 = capture
                        .name("x")
                        .context("x value not found")?
                        .as_str()
                        .parse()?;
                    let y: i64 = capture
                        .name("y")
                        .context("y value not found")?
                        .as_str()
                        .parse()?;
                    Ok((x, y))
                })
                .collect::<Result<Vec<(i64, i64)>>>()?;
            let prize_capture = prize_re
                .captures(game_input)
                .context("could not capture prize")?;
            let prize_x: i64 = prize_capture
                .name("x")
                .context("x value not found in prize")?
                .as_str()
                .parse()?;
            let prize_y: i64 = prize_capture
                .name("y")
                .context("y value not found in prize")?
                .as_str()
                .parse()?;
            Ok(Game {
                buttons,
                prize: (prize_x, prize_y),
            })
        })
        .collect()
}

fn part_one() -> Result<()> {
    let games = read_games_input()?;

    let result: i64 = games
        .into_iter()
        .map(|game| {
            let prize = game.prize;
            let button_a = game.buttons[0];
            let button_b = game.buttons[1];

            let mut min_score: Option<i64> = None;
            for a_count in 0..=100i64 {
                for b_count in 0..=100i64 {
                    let x = button_a.0 * a_count + button_b.0 * b_count;
                    let y = button_a.1 * a_count + button_b.1 * b_count;
                    if prize != (x, y) {
                        continue;
                    }

                    let current_score = a_count * 3 + b_count;
                    if min_score.is_none() {
                        min_score = Some(current_score);
                    } else {
                        min_score = min_score.min(Some(current_score));
                    }
                }
            }
            min_score.unwrap_or(0)
        })
        .sum();

    println!("part one result {}", result);

    Ok(())
}

fn solve_game_for_part_two(game: &Game) -> i64 {
    let diff: i64 = 10000000000000;
    let prize = (game.prize.0 + diff, game.prize.1 + diff);
    let button_a = game.buttons[0];
    let button_b = game.buttons[1];

    let upper_l = button_a.0 * prize.1 - button_a.1 * prize.0;
    let lower_l = button_a.0 * button_b.1 - button_a.1 * button_b.0;

    if upper_l % lower_l != 0 {
        return 0;
    }

    let l = upper_l / lower_l;

    let upper_k = prize.0 - button_b.0 * l;
    let lower_k = button_a.0;

    if upper_k % lower_k != 0 {
        return 0;
    }

    let k = upper_k / lower_k;

    let x = button_a.0 * k + button_b.0 * l;
    let y = button_a.1 * k + button_b.1 * l;

    if prize != (x, y) {
        return 0;
    }

    return 3 * k + l;
}

fn part_two() -> Result<()> {
    let games = read_games_input()?;

    let result: i64 = games
        .into_iter()
        .map(|game| solve_game_for_part_two(&game))
        .sum();

    println!("part two result {}", result);

    Ok(())
}

pub fn solve() -> Result<()> {
    part_two()
}
