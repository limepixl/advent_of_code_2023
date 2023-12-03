use std::{fs, error::Error };
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("input.txt")?;
    let lines = contents.lines();

    let map = HashMap::from([("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)]);

    let mut sum = 0;
    for line in lines {
        let mut first_loc = line.find(|c| char::is_digit(c, 10));
        let mut last_loc = line.rfind(|c| char::is_digit(c, 10));

        let mut first_digit = None;
        let mut last_digit = None;

        if let Some(loc) = first_loc {
            let char = line.chars().nth(loc);
            if let Some(char) = char {
                first_digit = Some(char::to_digit(char, 10).unwrap())
            }
        }

        if let Some(loc) = last_loc {
            let char = line.chars().nth(loc);
            if let Some(char) = char {
                last_digit = Some(char::to_digit(char, 10).unwrap())
            }
        }

        for key in map.keys() {
            if let Some(loc) = line.find(key) {
                if Some(loc) < first_loc {
                    first_loc = Some(loc);
                    first_digit = Some(*map.get(key).unwrap());
                }
            }

            if let Some(loc) = line.rfind(key) {
                if Some(loc) > last_loc {
                    last_loc = Some(loc);
                    last_digit = Some(*map.get(key).unwrap());
                }
            }
        }

        if let (Some(first), Some(last)) = (first_digit, last_digit) {
            let result = first * 10 + last;
            sum += result;

            println!("For string '{line}' the number is {result}");
        }
        else {
            panic!("Not implemented case where no digits are found!");
        }
    }

    println!("{sum}");
    return Ok(());
}
