use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::zip,
};

fn read_from_file(file_path: &str) -> (Vec<i32>, Vec<i32>) {
    let file = File::open(file_path).expect("file not found!");
    let reader = BufReader::new(file);

    let mut first = vec![];
    let mut second = vec![];

    reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split("   ")
                .map(|number| number.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .for_each(|numbers| {
            first.push(numbers[0]);
            second.push(numbers[1]);
        });

    (first, second)
}

fn part_one() {
    let numbers = read_from_file("./src/input.txt");

    let mut first = numbers.0.to_vec();
    let mut second = numbers.1.to_vec();

    first.sort();
    second.sort();

    let result = zip(first, second)
        .map(|value| (value.0 - value.1).abs())
        .fold(0, |a, b| a + b);

    print!("{}", result);
}

fn main() {
    part_one();
}
