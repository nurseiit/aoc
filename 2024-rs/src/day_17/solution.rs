use std::{fs::read_to_string, i64};

use anyhow::{Context, Result};

#[derive(Debug, Clone)]
struct Computer {
    registers: Vec<i64>,
    program: Vec<i64>,
    pointer: usize,
    output: Vec<i64>,
    early_exit: bool,
}

impl Computer {
    pub fn run(&mut self) {
        while self.run_instruction(false) {}
    }

    pub fn run_part_two(&self, a_value: i64) -> bool {
        let mut computer = self.clone();

        computer.registers[0] = a_value;
        while computer.run_instruction(true) {}

        !computer.early_exit && computer.output.len() == computer.program.len()
    }

    fn run_instruction(&mut self, should_match_program: bool) -> bool {
        if self.pointer >= self.program.len() {
            return false;
        }
        match self.get_opcode() {
            0 => {
                if self.pointer + 1 >= self.program.len() {
                    return false;
                }
                self.registers[0] = self.registers[0] >> self.get_combo_operand();
                self.pointer += 2;
                true
            }
            1 => {
                if self.pointer + 1 >= self.program.len() {
                    return false;
                }
                self.registers[1] = self.registers[1] ^ self.get_literal_operand();
                self.pointer += 2;
                true
            }
            2 => {
                if self.pointer + 1 >= self.program.len() {
                    return false;
                }
                self.registers[1] = self.get_combo_operand() & 7;
                self.pointer += 2;
                true
            }
            3 => {
                if self.registers[0] == 0 {
                    // do nothing
                    self.pointer += 2;
                    return true;
                }
                if self.pointer + 1 >= self.program.len() {
                    return false;
                }
                self.pointer = self.get_literal_operand() as usize;
                true
            }
            4 => {
                if self.pointer + 1 >= self.program.len() {
                    return false;
                }
                self.registers[1] = self.registers[1] ^ self.registers[2];
                self.pointer += 2;
                true
            }
            5 => {
                if self.pointer + 1 >= self.program.len() {
                    return false;
                }
                let len = self.output.len();
                self.output.push(self.get_combo_operand() & 7);
                // early exit for part two
                if should_match_program
                    && (self.program.len() < len || self.program[len] != self.output[len])
                {
                    self.early_exit = true;
                    return false;
                }
                self.pointer += 2;
                true
            }
            6 => {
                if self.pointer + 1 >= self.program.len() {
                    return false;
                }
                self.registers[1] = self.registers[0] >> self.get_combo_operand();
                self.pointer += 2;
                true
            }
            7 => {
                if self.pointer + 1 >= self.program.len() {
                    return false;
                }
                self.registers[2] = self.registers[0] >> self.get_combo_operand();
                self.pointer += 2;
                true
            }
            _ => false,
        }
    }

    fn get_opcode(&self) -> i64 {
        self.program[self.pointer]
    }

    fn get_combo_operand(&self) -> i64 {
        let operand = self.program[self.pointer + 1];
        match operand {
            0..=3 => operand,
            4 => self.registers[0],
            5 => self.registers[1],
            6 => self.registers[2],
            _ => panic!("unexpected operand '{}'", operand),
        }
    }

    fn get_literal_operand(&self) -> i64 {
        self.program[self.pointer + 1]
    }
}

fn read_computer(file_name: &str) -> Result<Computer> {
    let input = read_to_string(format!("./src/day_17/{}.txt", file_name))
        .context("could not read input file")?;
    let input_parts: Vec<&str> = input.split("\n\n").collect();

    let registers = input_parts
        .get(0)
        .context("could not get registers part of input")?
        .lines()
        .map(|register_line| {
            register_line
                .get(12..)
                .context("could not get register value")?
                .parse::<i64>()
                .context(format!("could not parse register value"))
        })
        .collect::<Result<Vec<i64>>>()?;

    let program = input_parts
        .get(1)
        .context("could not get program part of input")?
        .get(9..)
        .context("could not get program values")?
        .trim()
        .split(",")
        .map(|value| {
            value
                .parse::<i64>()
                .context(format!("could not parse program value '{}'", value))
        })
        .collect::<Result<Vec<i64>>>()?;

    Ok(Computer {
        registers,
        program,
        pointer: 0,
        output: vec![],
        early_exit: false,
    })
}

pub fn solve() -> Result<()> {
    let mut computer = read_computer("input")?;
    let computer_two = computer.clone();

    computer.run();

    let part_one: String = computer
        .output
        .iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>()
        .join(",");

    println!("part one result: {}", part_one);

    // let start: i64 = 0;
    let start: i64 = 16982000000;

    if let Some(part_two) = (start..i64::MAX).find(|&a_value| {
        if a_value % 1000000 == 0 {
            println!("checking {}", a_value);
        }
        computer_two.run_part_two(a_value)
    }) {
        println!("part two result: {}", part_two);
    }

    Ok(())
}
