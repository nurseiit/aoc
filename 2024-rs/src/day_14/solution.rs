use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::read_to_string,
    iter::repeat,
};

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

fn show_table(table: &Vec<Vec<bool>>) {
    for row in table {
        for cell in row {
            print!("{}", if *cell { "#" } else { "." })
        }
        println!("");
    }
}

fn get_valid_neighbours((i, j): (usize, usize), (n, m): (usize, usize)) -> Vec<(usize, usize)> {
    let dxdy: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut valids = vec![];

    for (dx, dy) in dxdy {
        let ni = dx + i as i32;
        let nj = dy + j as i32;
        if ni < 0 || ni >= n as i32 || nj < 0 || nj >= m as i32 {
            continue;
        }
        valids.push((ni as usize, nj as usize));
    }

    valids
}

fn check_tree(table: &Vec<Vec<bool>>, second: i32) -> bool {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut was: HashSet<(usize, usize)> = HashSet::new();

    let n = table.len();
    let m = table[0].len();
    let mut max_island = 0;

    for i in 0..n {
        for j in 0..m {
            if table[i][j] == false || was.contains(&(i, j)) {
                continue;
            }
            let mut curr_count = 0;
            queue.push_back((i, j));
            while let Some(top) = queue.pop_front() {
                curr_count += 1;
                was.insert(top);

                let neighours: Vec<(usize, usize)> = get_valid_neighbours(top, (n, m))
                    .into_iter()
                    .filter(|(x, y)| table[*x][*y] && !was.contains(&(*x, *y)))
                    .collect();

                neighours.into_iter().for_each(|(x, y)| {
                    was.insert((x, y));
                    queue.push_back((x, y));
                });
            }
            max_island = max_island.max(curr_count);
        }
    }

    if max_island < 30 {
        return false;
    }

    println!("Second: {}", second);
    show_table(&table);
    println!("");
    true
}

fn part_two() -> Result<()> {
    let robots = get_robots_data("input")?;

    // example dimensions
    // let (n, m) = (11, 7);
    // main dimensions
    let (n, m) = (101, 103);
    let mut table: Vec<Vec<bool>> = repeat(repeat(false).take(m).collect()).take(n).collect();

    for second in 0..10000 {
        println!("checking {}", second);
        let end_positions: Vec<(usize, usize)> = robots
            .iter()
            .map(|robot| {
                let end_x = (robot.position.0 + robot.velocity.0 * second).rem_euclid(n as i32);
                let end_y = (robot.position.1 + robot.velocity.1 * second).rem_euclid(m as i32);
                (end_x as usize, end_y as usize)
            })
            .collect();

        // set table
        end_positions.iter().for_each(|&(x, y)| table[x][y] = true);

        if check_tree(&table, second) {
            break;
        }

        // cleanup table
        end_positions.iter().for_each(|&(x, y)| table[x][y] = false);
    }

    Ok(())
}

pub fn solve() -> Result<()> {
    part_two()
}
