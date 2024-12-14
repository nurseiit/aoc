use std::fs::read_to_string;

use anyhow::{Context, Result};
use regex::Regex;

#[derive(Debug, Clone)]
struct Game {
    buttons: Vec<(i64, i64)>,
    prize: (i64, i64),
}

fn read_games_input() -> Result<Vec<Game>> {
    let input = read_to_string("./src/day_13/example.txt").context("could not read file")?;

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

fn gcd_ext(a: i64, b: i64, x: &mut i64, y: &mut i64) -> i64 {
    if b == 0 {
        *x = 1;
        *y = 0;
        a
    } else {
        let mut x1 = 0;
        let mut y1 = 0;
        let g = gcd_ext(b, a % b, &mut x1, &mut y1);
        *x = y1;
        *y = x1 - y1 * (a / b);
        g
    }
}

fn solve_game(game: &Game) -> i64 {
    let diff: i64 = 10000000000000;
    // let diff: i64 = 0;
    let prize = (game.prize.0 + diff, game.prize.1 + diff);
    let button_a = game.buttons[0];
    let button_b = game.buttons[1];

    let aa = button_a.0;
    let ab = button_b.0;

    let ba = button_a.1;
    let bb = button_b.1;

    let mut ax = 0;
    let mut ay = 0;

    let mut bx = 0;
    let mut by = 0;

    let ga = gcd_ext(aa, ab, &mut ax, &mut ay);
    let gb = gcd_ext(ba, bb, &mut bx, &mut by);

    if prize.0 % ga != 0 || prize.1 % gb != 0 {
        return 0;
    }

    let da = prize.0 / ga;
    let db = prize.1 / gb;

    ax *= da;
    ay *= da;

    bx *= db;
    by *= db;

    let ax_step = ab / ga;
    let ay_step = aa / ga;

    let bx_step = bb / gb;
    let by_step = ba / gb;

    // let a_mid_k = -ax / ax_step;
    // let b_mid_k = -bx / bx_step;
    let a_mid_k = 0;
    let b_mid_k = 0;

    // println!(
    //     "Solved: {} = {} * {} + {} * {}",
    //     prize.0,
    //     aa,
    //     ax + a_mid_k * ax_step,
    //     ab,
    //     ay - a_mid_k * ay_step
    // );
    //
    // println!(
    //     "Solved: {} = {} * {} + {} * {}\n",
    //     prize.1,
    //     ba,
    //     bx + b_mid_k * bx_step,
    //     bb,
    //     by - b_mid_k * by_step
    // );

    let mut min_score: Option<i64> = None;

    let mut k = 1;
    let mut op = 0;
    let max_ops = 2;
    while op < max_ops {
        let aa_count = ax + (a_mid_k + k) * ax_step;
        let ab_count = ay - (a_mid_k + k) * ay_step;

        let x = button_a.0 * aa_count + button_b.0 * ab_count;
        let y = button_a.1 * aa_count + button_b.1 * ab_count;

        println!("checking a count {} & b count {}", aa_count, ab_count);
        println!(
            "(exp {}) {} = {} * {} + {} * {} AND (exp {}) {} = {} * {} + {} * {}",
            prize.0,
            x,
            button_a.0,
            aa_count,
            button_b.0,
            ab_count,
            prize.1,
            y,
            button_a.1,
            aa_count,
            button_b.1,
            ab_count
        );

        if prize == (x, y) && aa_count >= 0 && ab_count >= 0 {
            let current_score = aa_count * 3 + ab_count;
            if min_score.is_none() {
                min_score = Some(current_score);
            } else {
                min_score = min_score.min(Some(current_score));
            }
        }
        op += 1;
        if aa > ab {
            k -= 1;
        } else {
            k += 1;
        }
    }

    println!("contd");

    k = 1;
    op = 0;
    while op < max_ops {
        let ba_count = bx + (b_mid_k + k) * bx_step;
        let bb_count = by - (b_mid_k + k) * by_step;

        println!("checking a count {} & b count {}", ba_count, bb_count);

        let x = button_a.0 * ba_count + button_b.0 * bb_count;
        let y = button_a.1 * ba_count + button_b.1 * bb_count;

        println!(
            "(exp {}) {} = {} * {} + {} * {} AND (exp {}) {} = {} * {} + {} * {}",
            prize.0,
            x,
            button_a.0,
            ba_count,
            button_b.0,
            bb_count,
            prize.1,
            y,
            button_a.1,
            ba_count,
            button_b.1,
            bb_count
        );

        if prize == (x, y) && ba_count >= 0 && bb_count >= 0 {
            let current_score = ba_count * 3 + bb_count;
            if min_score.is_none() {
                min_score = Some(current_score);
            } else {
                min_score = min_score.min(Some(current_score));
            }
        }
        op += 1;
        if ba > bb {
            k -= 1;
        } else {
            k += 1;
        }
    }

    println!("====");

    min_score.unwrap_or(0)
}

fn part_two() -> Result<()> {
    let games = read_games_input()?;

    let result: i64 = games.into_iter().map(|game| solve_game(&game)).sum();

    println!("part two result {}", result);

    Ok(())
}

pub fn solve() -> Result<()> {
    part_two()
}
