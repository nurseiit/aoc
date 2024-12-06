use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::read_to_string,
};

use anyhow::{Context, Error, Result};

struct Data {
    rules: Vec<(i32, i32)>,
    updates: Vec<Vec<i32>>,
}

fn read_data_from_file(file_path: &str) -> Result<Data, Error> {
    let input = read_to_string(file_path).context("could not read file")?;

    let input_parts: Vec<&str> = input.split("\n\n").collect();
    let rules_part = *input_parts
        .get(0)
        .context("could not get rules part from input")?;
    let updates_part = *input_parts
        .get(1)
        .context("could not get updates part from input")?;

    let rules = rules_part
        .lines()
        .map(|line| {
            line.split("|")
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|nums| (nums[0], nums[1]))
        .collect();

    let updates = updates_part
        .lines()
        .map(|line| {
            line.split(",")
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    Ok(Data { rules, updates })
}

fn part_one() -> Result<(), Error> {
    let data = read_data_from_file("./src/day_05/input.txt")?;

    println!(
        "rules len {}, updates len {}",
        data.rules.len(),
        data.updates.len()
    );

    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();

    data.rules.iter().for_each(|&(from, to)| {
        graph.entry(from).or_insert(HashSet::new()).insert(to);
    });

    let result: i32 = data
        .updates
        .into_iter()
        .filter(|update| {
            let mut was: HashSet<i32> = HashSet::new();

            let allowlist: HashSet<i32> = HashSet::from_iter(update.iter().cloned());

            update.iter().rev().all(|&current| {
                if was.contains(&current) {
                    return false;
                }
                let mut queue: VecDeque<i32> = VecDeque::new();
                queue.push_back(current);

                while let Some(from) = queue.pop_front() {
                    was.insert(from);
                    graph
                        .entry(from)
                        .or_insert(HashSet::new())
                        .iter()
                        .filter(|to| !was.contains(*to) && allowlist.contains(*to))
                        .for_each(|to| queue.push_back(*to));
                }
                return true;
            })
        })
        .map(|update| update[update.len() / 2])
        .sum();

    println!("valid ones {}", result);

    Ok(())
}

pub fn solve() -> Result<(), Error> {
    part_one()
}
