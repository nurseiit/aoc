pub mod solution {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    use regex::Regex;

    fn read_file(file_path: &str) -> String {
        let file = File::open(file_path).expect("could not read file");
        let reader = BufReader::new(file);

        reader
            .lines()
            .map(|line| line.expect("could not read line"))
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn part_one() {
        let data = read_file("./src/day_03/input.txt");
        let re = Regex::new(r"mul\((?<left>\d+)\,(?<right>\d+)\)").unwrap();

        let result = re
            .captures_iter(data.as_str())
            .map(|capture| {
                let left: &str = capture.name("left").expect("left not found").into();
                let right: &str = capture.name("right").expect("right not found").into();
                (left, right)
            })
            .map(|(left, right)| (left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap()))
            .fold(0, |acc, (left, right)| acc + left * right);

        println!("part one result {}", result);
    }

    pub fn solve() {
        part_one();
    }
}
