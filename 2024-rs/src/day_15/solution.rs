use anyhow::{Context, Error, Ok, Result};
use std::fs::read_to_string;

#[derive(Debug)]
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

fn part_one() -> Result<()> {
    let doc = read_document("simple")?;

    println!("doc: {:?}", doc);

    Ok(())
}

pub fn solve() -> Result<()> {
    part_one()
}
