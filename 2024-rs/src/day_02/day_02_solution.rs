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

    fn is_increasing(nums: &[i32]) -> bool {
        nums.windows(2).all(|window| window[0] < window[1])
    }

    fn is_decreasing(nums: &[i32]) -> bool {
        nums.windows(2).all(|window| window[0] > window[1])
    }

    fn is_within_diff_range(nums: &[i32]) -> bool {
        nums.windows(2).all(|window| {
            let diff = (window[0] - window[1]).abs();
            (1..=3).contains(&diff)
        })
    }

    fn part_one() {
        let data = read_from_file("./src/day_02/input.txt");
        let result = data
            .iter()
            .filter(|nums| {
                (is_increasing(nums) || is_decreasing(nums)) && is_within_diff_range(nums)
            })
            .count();

        println!("part one: {}", result);
        assert_eq!(result, 421);
    }

    fn is_tolerable_ok(nums: &[i32]) -> bool {
        nums.iter().enumerate().any(|(i, _)| {
            let mut copy = nums.to_vec();
            copy.remove(i);
            (is_increasing(&copy) || is_decreasing(&copy)) && is_within_diff_range(&copy)
        })
    }

    fn part_two() {
        let data = read_from_file("./src/day_02/input.txt");
        let result = data.iter().filter(|nums| is_tolerable_ok(nums)).count();
        println!("part two: {}", result);
        assert_eq!(result, 476);
    }

    pub fn solve() {
        part_one();
        part_two();
    }
}
