use std::fs::read_to_string;

use anyhow::{Context, Result};

#[derive(Debug, Clone)]
struct Computer {
    registers: Vec<i32>,
    program: Vec<i32>,
    pointer: usize,
    output: Vec<i32>,
}

impl Computer {
    pub fn run(&mut self) {
        while self.run_instruction(false) {}
    }

    pub fn run_part_two(&self, a_value: i32) -> bool {
        let mut computer = self.clone();

        computer.registers[0] = a_value;
        while computer.run_instruction(true) {}

        if computer.output.len() != computer.program.len() {
            return false;
        }

        computer
            .output
            .iter()
            .zip(computer.program.iter())
            .all(|(&output, &program)| output == program)
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
                self.registers[0] = self.registers[0] / (1 << self.get_combo_operand());
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
                self.registers[1] = self.get_combo_operand() % 8;
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
                self.output.push(self.get_combo_operand() % 8);
                // early exit for part two
                if should_match_program
                    && (self.program.len() < len || self.program[len] != self.output[len])
                {
                    return false;
                }
                self.pointer += 2;
                true
            }
            6 => {
                if self.pointer + 1 >= self.program.len() {
                    return false;
                }
                self.registers[1] = self.registers[0] / (1 << self.get_combo_operand());
                self.pointer += 2;
                true
            }
            7 => {
                if self.pointer + 1 >= self.program.len() {
                    return false;
                }
                self.registers[2] = self.registers[0] / (1 << self.get_combo_operand());
                self.pointer += 2;
                true
            }
            _ => false,
        }
    }

    fn get_opcode(&self) -> i32 {
        self.program[self.pointer]
    }

    fn get_combo_operand(&self) -> i32 {
        let operand = self.program[self.pointer + 1];
        match operand {
            0..=3 => operand,
            4 => self.registers[0],
            5 => self.registers[1],
            6 => self.registers[2],
            _ => panic!("unexpected operand '{}'", operand),
        }
    }

    fn get_literal_operand(&self) -> i32 {
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
                .parse::<i32>()
                .context(format!("could not parse register value"))
        })
        .collect::<Result<Vec<i32>>>()?;

    let program = input_parts
        .get(1)
        .context("could not get program part of input")?
        .get(9..)
        .context("could not get program values")?
        .trim()
        .split(",")
        .map(|value| {
            value
                .parse::<i32>()
                .context(format!("could not parse program value '{}'", value))
        })
        .collect::<Result<Vec<i32>>>()?;

    Ok(Computer {
        registers,
        program,
        pointer: 0,
        output: vec![],
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

    if let Some(part_two) = (50000000..10000000000).find(|&a_value| {
        if a_value % 1000 == 0 {
            println!("checking {}", a_value);
        }
        computer_two.run_part_two(a_value)
    }) {
        println!("part two result: {}", part_two);
    }

    Ok(())
}
