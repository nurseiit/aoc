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

    fn is_increasing(nums: Vec<i32>) -> bool {
        for i in 1..nums.len() {
            if nums[i - 1] >= nums[i] {
                return false;
            }
        }
        true
    }

    fn is_decreasing(nums: Vec<i32>) -> bool {
        for i in 1..nums.len() {
            if nums[i - 1] <= nums[i] {
                return false;
            }
        }
        true
    }

    fn is_within_diff_range(nums: Vec<i32>) -> bool {
        for i in 1..nums.len() {
            let diff = (nums[i] - nums[i - 1]).abs();
            if diff < 1 || diff > 3 {
                return false;
            }
        }
        true
    }

    fn part_one() {
        let data = read_from_file("./src/day_02/input.txt");
        let result = data
            .iter()
            .filter(|nums| {
                (is_increasing(nums.to_vec()) || is_decreasing(nums.to_vec()))
                    && is_within_diff_range(nums.to_vec())
            })
            .count();

        println!("part one: {}", result);
    }

    fn is_tolerable_ok(nums: Vec<i32>) -> bool {
        for i in 0..nums.len() {
            let mut copy = nums.to_vec();
            copy.remove(i);
            if (is_increasing(copy.to_vec()) || is_decreasing(copy.to_vec()))
                && is_within_diff_range(copy.to_vec())
            {
                return true;
            }
        }
        false
    }

    fn part_two() {
        let data = read_from_file("./src/day_02/input.txt");
        let result = data
            .iter()
            .filter(|nums| is_tolerable_ok(nums.to_vec()))
            .count();
        println!("part two: {}", result);
    }

    pub fn solve() {
        part_one();
        part_two();
    }
}
