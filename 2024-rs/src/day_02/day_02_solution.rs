pub mod solution {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    fn read_from_file(file_path: &str) -> Vec<Vec<i32>> {
        let file = File::open(file_path).expect("file not found!");
        let reader = BufReader::new(file);

        reader
            .lines()
            .map(|line| {
                line.unwrap()
                    .split(" ")
                    .map(|item| item.parse::<i32>().unwrap())
                    .collect()
            })
            .collect()
    }

    fn part_one() {
        let data = read_from_file("./src/day_02/example.txt");
        assert_eq!(data[0], vec![7, 6, 4, 2, 1]);
        assert_eq!(data.len(), 6);
    }

    pub fn solve() {
        part_one();
    }
}
