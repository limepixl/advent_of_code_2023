use std::{fs, collections::HashMap};
use regex::{self, Regex};

fn main() {
    let string = fs::read_to_string("input.txt").unwrap();
    let lines = string.lines();

    let bag = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let mut sum_powers = 0;
    let mut sum_of_ids = 0;

    let configs_regex = Regex::new(r"\d+ (?:green|blue|red) *").unwrap();

    for mut line in lines {
        let split: Vec<&str> = line.split(": ").collect();
        line = split[1];
        
        let id = split[0].split("Game ")
                              .last().unwrap()
                              .parse::<i32>().unwrap();

        let mut is_possible = true;
        let configurations = line.split("; ");
        let mut min_num_cubes = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

        for config in configurations {
            let captured: Vec<(i32, &str)> = configs_regex.find_iter(config).map(|mtch| {
                let parts: Vec<_> = mtch.as_str().split(" ").collect();
                let count = parts[0].parse::<i32>().unwrap();
                (count, parts[1])
            }).collect();

            for (count, color) in captured {
                if let Some(&existing_count) = min_num_cubes.get(color) {
                    let min_count = if count > existing_count { count } else { existing_count };
                    min_num_cubes.insert(color, min_count);
                }

                if let Some(&max_count) = bag.get(color) {
                    is_possible = is_possible && count <= max_count;
                }
            }
        }
        sum_powers += min_num_cubes.values().product::<i32>();
        sum_of_ids += if is_possible { id } else { 0 };
    }

    println!("Sum of ids: {sum_of_ids}");
    println!("Sum of powers: {sum_powers}");
}
