use std::fs::read_to_string;

use anyhow::{Context, Result};
use regex::Regex;

#[derive(Debug, Clone)]
struct Game {
    buttons: Vec<(u32, u32)>,
    prize: (u32, u32),
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
                    let x: u32 = capture
                        .name("x")
                        .context("x value not found")?
                        .as_str()
                        .parse()?;
                    let y: u32 = capture
                        .name("y")
                        .context("y value not found")?
                        .as_str()
                        .parse()?;
                    Ok((x, y))
                })
                .collect::<Result<Vec<(u32, u32)>>>()?;
            let prize_capture = prize_re
                .captures(game_input)
                .context("could not capture prize")?;
            let prize_x: u32 = prize_capture
                .name("x")
                .context("x value not found in prize")?
                .as_str()
                .parse()?;
            let prize_y: u32 = prize_capture
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

    let scores = games
        .into_iter()
        .map(|game| {
            let prize = game.prize;
            let button_a = game.buttons.get(0).context("could not get button A")?;
            let button_b = game.buttons.get(1).context("could not get button B")?;

            let mut min_score: Option<u32> = None;
            for a_count in 0..=100u32 {
                for b_count in 0..=100u32 {
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
            Ok(min_score.unwrap_or(0))
        })
        .collect::<Result<Vec<u32>>>()?;

    println!("part one result {}", scores.into_iter().sum::<u32>());

    Ok(())
}

pub fn solve() -> Result<()> {
    part_one()
}
