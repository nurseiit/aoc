use std::{collections::HashMap, fs::read_to_string};

use anyhow::{Context, Result};

#[derive(Debug)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

fn get_robots_data(file_name: &str) -> Result<Vec<Robot>> {
    let input =
        read_to_string(format!("./src/day_14/{}.txt", file_name)).context("could not read file")?;
    input
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split(" ").collect();
            let p_part = parts
                .get(0)
                .context("could not get position part")?
                .get(2..)
                .context("could not get position data part")?
                .split(",")
                .map(|p| {
                    p.parse::<i32>()
                        .context(format!("could not parse p '{}' to i32", p))
                })
                .collect::<Result<Vec<i32>>>()?;
            let v_part = parts
                .get(1)
                .context("could not get velocity part")?
                .get(2..)
                .context("could not get velocity data part")?
                .split(",")
                .map(|v| {
                    v.parse::<i32>()
                        .context(format!("could not parse v '{}' to i32", v))
                })
                .collect::<Result<Vec<i32>>>()?;

            let position: (i32, i32) = (
                *p_part.get(0).context("could not get first position")?,
                *p_part.get(1).context("could not get second position")?,
            );

            let velocity: (i32, i32) = (
                *v_part.get(0).context("could not get first velocity")?,
                *v_part.get(1).context("could not get second velocity")?,
            );

            Ok(Robot { position, velocity })
        })
        .collect()
}

fn part_one() -> Result<()> {
    let robots = get_robots_data("input")?;

    let seconds = 100;
    // example dimensions
    // let (n, m) = (11, 7);
    // main dimensions
    let (n, m) = (101, 103);

    let end_positions: Vec<(i32, i32)> = robots
        .into_iter()
        .map(|robot| {
            let end_x = (robot.position.0 + robot.velocity.0 * seconds).rem_euclid(n);
            let end_y = (robot.position.1 + robot.velocity.1 * seconds).rem_euclid(m);
            (end_x, end_y)
        })
        .collect();

    let (mid_n, mid_m) = (n / 2, m / 2);
    let mut quadrants: HashMap<(i32, i32), i32> = HashMap::new();

    end_positions
        .into_iter()
        .filter(|&(x, y)| x != mid_n && y != mid_m)
        .for_each(|(x, y)| {
            *quadrants
                .entry((if x < mid_n { 0 } else { 1 }, if y < mid_m { 0 } else { 1 }))
                .or_insert(0) += 1
        });

    let result: i32 = quadrants.values().product();

    println!("part one result {}", result);

    Ok(())
}

pub fn solve() -> Result<()> {
    part_one()
}
