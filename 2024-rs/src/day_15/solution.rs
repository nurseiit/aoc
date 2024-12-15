use anyhow::{Context, Error, Ok, Result};
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq)]
enum TableItem {
    WALL,
    ROBOT,
    BOX,
    EMPTY,
}

#[derive(Debug)]
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
        TableItem::WALL => (),
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
    }
}

fn part_one() -> Result<()> {
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

pub fn solve() -> Result<()> {
    part_one()
}
