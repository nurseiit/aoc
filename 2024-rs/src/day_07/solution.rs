use std::fs::read_to_string;

use anyhow::{Context, Error, Result};

#[derive(Debug, Clone)]
struct Equation {
    result: i64,
    operands: Vec<i64>,
}

fn read_equations_from_file(file_path: &str) -> Result<Vec<Equation>, Error> {
    let input = read_to_string(file_path).context("could not read file")?;
    let equations: Result<Vec<Equation>, _> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            let result = parts
                .get(0)
                .context("could not get result part")?
                .parse::<i64>()
                .context("could not convert result part to i64")?;
            let operands: Result<Vec<i64>, _> = parts
                .get(1)
                .context("could not get operands part")?
                .split(" ")
                .map(|operand| {
                    operand
                        .parse::<i64>()
                        .context("could not convert operand to i64")
                })
                .collect();
            Ok::<Equation, Error>(Equation {
                result,
                operands: operands?,
            })
        })
        .collect();

    Ok(equations?)
}

fn check_if_equation_is_true(equation: Equation) -> bool {
    let num_of_operators = equation.operands.len() - 1;
    let mask_end = 1 << num_of_operators;
    (0..mask_end).any(|mask| {
        equation.result
            == equation
                .operands
                .iter()
                .enumerate()
                .fold(0i64, |acc, (i, num)| {
                    if i == 0 || mask & (1 << (i - 1)) != 0 {
                        acc + num
                    } else {
                        acc * num
                    }
                })
    })
}

fn part_one() -> Result<(), Error> {
    let equations = read_equations_from_file("./src/day_07/input.txt")?;

    let result: i64 = equations
        .into_iter()
        .filter(|eq| check_if_equation_is_true(eq.clone()))
        .map(|eq| eq.result)
        .sum();

    println!("part one result {}", result);

    Ok(())
}

pub fn solve() -> Result<(), Error> {
    part_one()
}
