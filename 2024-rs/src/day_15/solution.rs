use anyhow::{Context, Error, Ok, Result};
use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};

#[derive(Debug, PartialEq, Eq, Clone)]
enum TableItem {
    WALL,
    ROBOT,
    BOX,
    BoxLeft,
    BoxRight,
    EMPTY,
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug)]
struct Document {
    table: Vec<Vec<TableItem>>,
    moves: Vec<Direction>,
}

fn read_document(file_name: &str) -> Result<Document> {
    let input =
        read_to_string(format!("./src/day_15/{}.txt", file_name)).context("could not read file")?;
    let input_parts: Vec<&str> = input.split("\n\n").collect();

    let table_raw: Vec<Vec<char>> = input_parts
        .get(0)
        .context("could not get table part of input")?
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let table = table_raw
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|cell| match cell {
                    '#' => Ok(TableItem::WALL),
                    '@' => Ok(TableItem::ROBOT),
                    '.' => Ok(TableItem::EMPTY),
                    'O' => Ok(TableItem::BOX),
                    _ => Err(Error::msg(format!("unknown table item '{}'", cell))),
                })
                .collect()
        })
        .collect::<Result<Vec<Vec<TableItem>>>>()?;

    let moves_raw: Vec<char> = input_parts
        .get(1)
        .context("could not get moves part of input")?
        .lines()
        .flat_map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let moves = moves_raw
        .into_iter()
        .map(|direction| match direction {
            '^' => Ok(Direction::UP),
            '>' => Ok(Direction::RIGHT),
            'v' => Ok(Direction::DOWN),
            '<' => Ok(Direction::LEFT),
            _ => Err(Error::msg(format!("unknown direction '{}'", direction))),
        })
        .collect::<Result<Vec<Direction>>>()?;

    Ok(Document { table, moves })
}

fn get_robot_position(table: &Vec<Vec<TableItem>>) -> (usize, usize) {
    for i in 0..table.len() {
        for j in 0..table[i].len() {
            if table[i][j] == TableItem::ROBOT {
                return (i, j);
            }
        }
    }
    (0, 0)
}

fn part_one() -> Result<()> {
    fn move_in_direction(direction: &Direction, table: &mut Vec<Vec<TableItem>>) {
        let (dx, dy) = match *direction {
            Direction::UP => (-1, 0),
            Direction::DOWN => (1, 0),
            Direction::LEFT => (0, -1),
            Direction::RIGHT => (0, 1),
        };
        let (i, j) = get_robot_position(&table);
        let (nx, ny) = (i as i32 + dx, j as i32 + dy);

        let n = table.len() as i32;
        let m = table[0].len() as i32;

        if nx < 0 || nx >= n || ny < 0 || ny >= m {
            return;
        }

        let (ni, nj) = (nx as usize, ny as usize);

        match table[ni][nj] {
            TableItem::EMPTY => {
                table[i][j] = TableItem::EMPTY;
                table[ni][nj] = TableItem::ROBOT;
            }
            TableItem::BOX => {
                let mut ci = ni;
                let mut cj = nj;
                while table[ci][cj] != TableItem::WALL && table[ci][cj] != TableItem::EMPTY {
                    ci = (ci as i32 + dx) as usize;
                    cj = (cj as i32 + dy) as usize;
                }
                if table[ci][cj] == TableItem::EMPTY {
                    table[i][j] = TableItem::EMPTY;
                    table[ni][nj] = TableItem::ROBOT;
                    table[ci][cj] = TableItem::BOX;
                }
            }
            TableItem::ROBOT => panic!("found multiple robots!"),
            _ => (),
        }
    }
    let doc = read_document("input")?;

    let mut table = doc.table;
    let moves = doc.moves;

    for direction in moves {
        move_in_direction(&direction, &mut table);
    }

    let mut result: i32 = 0;

    for i in 0..table.len() {
        for j in 0..table[i].len() {
            if table[i][j] != TableItem::BOX {
                continue;
            }
            result += i as i32 * 100 + j as i32;
        }
    }

    println!("part one result {}", result);

    Ok(())
}

fn show_table(table: &Vec<Vec<TableItem>>) {
    for row in table {
        for cell in row {
            print!(
                "{}",
                match *cell {
                    TableItem::WALL => "#",
                    TableItem::ROBOT => "@",
                    TableItem::BOX => "O",
                    TableItem::BoxLeft => "[",
                    TableItem::BoxRight => "]",
                    TableItem::EMPTY => ".",
                }
            );
        }
        println!("");
    }
}

fn part_two() -> Result<()> {
    fn get_wide_table(table: &Vec<Vec<TableItem>>) -> Vec<Vec<TableItem>> {
        table
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .flat_map(|cell| match *cell {
                        TableItem::WALL => vec![TableItem::WALL, TableItem::WALL],
                        TableItem::ROBOT => vec![TableItem::ROBOT, TableItem::EMPTY],
                        TableItem::BOX => vec![TableItem::BoxLeft, TableItem::BoxRight],
                        TableItem::EMPTY => vec![TableItem::EMPTY, TableItem::EMPTY],
                        _ => vec![],
                    })
                    .collect()
            })
            .collect()
    }

    fn move_in_direction(direction: &Direction, table: &mut Vec<Vec<TableItem>>) {
        let (dx, dy) = match *direction {
            Direction::UP => (-1, 0),
            Direction::DOWN => (1, 0),
            Direction::LEFT => (0, -1),
            Direction::RIGHT => (0, 1),
        };
        let (i, j) = get_robot_position(&table);
        let (nx, ny) = (i as i32 + dx, j as i32 + dy);

        let n = table.len() as i32;
        let m = table[0].len() as i32;

        if nx < 0 || nx >= n || ny < 0 || ny >= m {
            return;
        }

        let (ni, nj) = (nx as usize, ny as usize);

        match table[ni][nj] {
            TableItem::EMPTY => {
                table[i][j] = TableItem::EMPTY;
                table[ni][nj] = TableItem::ROBOT;
            }
            TableItem::BoxLeft | TableItem::BoxRight => {
                let is_horizontal = vec![Direction::LEFT, Direction::RIGHT].contains(direction);

                if is_horizontal {
                    let mut cj = nj;
                    let mut positions: Vec<usize> = vec![cj];
                    while table[i][cj] != TableItem::WALL && table[i][cj] != TableItem::EMPTY {
                        cj = (cj as i32 + dy) as usize;
                        positions.push(cj);
                    }
                    if table[i][cj] == TableItem::WALL {
                        return;
                    }

                    positions.windows(2).rev().for_each(|window| {
                        table[i][window[1]] = table[i][window[0]].clone();
                    });
                    table[i][j] = TableItem::EMPTY;
                    table[ni][nj] = TableItem::ROBOT;
                    return;
                }
                // vertical
                let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
                let mut was: HashSet<(usize, usize)> = HashSet::new();
                queue.push_back((ni, j));

                if table[ni][j] == TableItem::BoxLeft {
                    queue.push_back((ni, j + 1));
                } else if table[ni][j] == TableItem::BoxRight {
                    queue.push_back((ni, j - 1));
                }

                while let Some(top) = queue.pop_front() {
                    was.insert(top);
                    let (top_i, top_j) = top;
                    let ci = (top_i as i32 + dx) as usize;

                    match table[ci][top_j] {
                        TableItem::WALL | TableItem::EMPTY => {}
                        TableItem::BoxLeft => {
                            if !was.contains(&(ci, top_j + 1)) {
                                was.insert((ci, top_j + 1));
                                queue.push_back((ci, top_j + 1));
                            }
                            if !was.contains(&(ci, top_j)) {
                                was.insert((ci, top_j));
                                queue.push_back((ci, top_j));
                            }
                        }
                        TableItem::BoxRight => {
                            if !was.contains(&(ci, top_j - 1)) {
                                was.insert((ci, top_j - 1));
                                queue.push_back((ci, top_j - 1));
                            }
                            if !was.contains(&(ci, top_j)) {
                                was.insert((ci, top_j));
                                queue.push_back((ci, top_j));
                            }
                        }
                        _ => {}
                    }
                }

                let can_move = was.iter().all(|&(ci, cj)| {
                    let nci = ci as i32 + dx;
                    nci >= 0 && nci < n && table[nci as usize][cj] != TableItem::WALL
                });

                if can_move {
                    let table_before = table.clone();
                    was.iter()
                        .for_each(|&(ci, cj)| table[ci][cj] = TableItem::EMPTY);
                    was.iter().for_each(|&(ci, cj)| {
                        let nci = ci as i32 + dx;
                        table[nci as usize][cj] = table_before[ci][cj].clone();
                    });
                    table[i][j] = TableItem::EMPTY;
                    table[ni][nj] = TableItem::ROBOT;
                }
            }
            _ => (),
        }
    }

    let doc = read_document("input")?;

    let mut table = get_wide_table(&doc.table);
    let moves = doc.moves;

    // println!("Initial:");
    // show_table(&table);
    // let mut line: String = Default::default();

    for i in 0..moves.len() {
        // let _ = std::io::stdin().read_line(&mut line);
        move_in_direction(&moves[i], &mut table);
        // println!("Move {}: {:?}", i, moves[i]);
        // show_table(&table);
    }

    let mut result: i32 = 0;

    for i in 0..table.len() {
        for j in 0..table[i].len() {
            if table[i][j] != TableItem::BoxLeft {
                continue;
            }
            result += i as i32 * 100 + j as i32;
        }
    }

    // println!("End:");
    // show_table(&table);

    println!("part two result {}", result);

    Ok(())
}

pub fn solve() -> Result<()> {
    part_two()
}
