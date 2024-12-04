pub mod solution {
    use anyhow::{Context, Error};
    use regex::Regex;
    use std::fs::read_to_string;

    fn part_one() -> Result<(), Error> {
        let data = read_to_string("./src/day_03/input.txt")?;
        let re = Regex::new(r"mul\((?<left>\d+)\,(?<right>\d+)\)")?;

        let result: i32 = re
            .captures_iter(data.as_str())
            .map(|capture| {
                let left: i32 = capture
                    .name("left")
                    .context("left not found")?
                    .as_str()
                    .parse()?;
                let right: i32 = capture
                    .name("right")
                    .context("right not found")?
                    .as_str()
                    .parse()?;
                Ok(left * right)
            })
            .collect::<Result<Vec<i32>, Error>>()?
            .into_iter()
            .sum();

        println!("part one result {}", result);

        Ok(())
    }

    fn part_two() -> Result<(), Error> {
        let data = read_to_string("./src/day_03/input.txt")?;
        let re = Regex::new(r"mul\((?<left>\d+)\,(?<right>\d+)\)|do\(\)|don\'t\(\)")?;

        let mut is_doing = true;
        let mut result = 0;

        re.captures_iter(data.as_str()).for_each(|capture| {
            let slice = capture.get(0).unwrap().as_str();

            match slice {
                "don't()" => {
                    is_doing = false;
                    return;
                }
                "do()" => {
                    is_doing = true;
                    return;
                }
                _ if !is_doing => {
                    return;
                }
                _ => {
                    let left: i32 = capture
                        .name("left")
                        .expect("left not found")
                        .as_str()
                        .parse::<i32>()
                        .unwrap();
                    let right: i32 = capture
                        .name("right")
                        .expect("right not found")
                        .as_str()
                        .parse::<i32>()
                        .unwrap();
                    result += left * right;
                }
            }
        });

        println!("part two result {}", result);

        Ok(())
    }

    pub fn solve() {
        let _ = part_one();
        let _ = part_two();
    }
}
