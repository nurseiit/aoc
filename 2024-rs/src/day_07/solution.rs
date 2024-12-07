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

fn combine(mut a: i64, b: i64) -> i64 {
    let mut temp = b;
    loop {
        a *= 10;
        temp /= 10;
        if temp == 0 {
            break;
        }
    }
    a + b
}

fn check_if_equation_is_true_with_combine(equation: Equation) -> bool {
    let num_of_operators = 2 * (equation.operands.len() - 1);
    let mask_end = 1 << num_of_operators;

    enum Op {
        Add,
        Combine,
        Mult,
    }

    fn get_mask_op(mask: i32, index: usize) -> Op {
        let double = index * 2;
        let first_bit = mask & (1 << double) != 0;
        let second_bit = mask & (1 << (double + 1)) != 0;

        if !first_bit && !second_bit {
            return Op::Add;
        }
        if !first_bit && second_bit {
            return Op::Mult;
        }
        Op::Combine
    }

    (0..mask_end).any(|mask| {
        equation.result
            == equation
                .operands
                .iter()
                .enumerate()
                .fold(0i64, |acc, (i, num)| {
                    let op = if i == 0 {
                        Op::Add
                    } else {
                        get_mask_op(mask, i - 1)
                    };
                    match op {
                        Op::Add => acc + num,
                        Op::Mult => acc * num,
                        Op::Combine => combine(acc, *num),
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

fn part_two() -> Result<(), Error> {
    let equations = read_equations_from_file("./src/day_07/input.txt")?;

    let result: i64 = equations
        .into_iter()
        .filter(|eq| check_if_equation_is_true_with_combine(eq.clone()))
        .map(|eq| eq.result)
        .sum();

    println!("part two result {}", result);

    Ok(())
}

pub fn solve() -> Result<(), Error> {
    part_two()
}
