use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    // day_01_part1("demo_input_part1.txt")?;
    day_01_part2("input.txt")?;
    Ok(())
}

pub fn day_01_part1(filepath: &str) -> std::io::Result<()>{
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for result_line in reader.lines() {
        let line = result_line?;
        for c in line.chars() {
            if c.is_numeric() {
                sum += c.to_digit(10).unwrap() * 10;
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_numeric() {
                sum += c.to_digit(10).unwrap();
                break;
            }
        }
    }
    println!("Result is {}", sum);
    Ok(())
}

pub fn day_01_part2(filepath: &str) -> std::io::Result<()>{
    let num_words = ["one", 
                     "two", 
                     "three", 
                     "four",
                     "five",
                     "six",
                     "seven",
                     "eight",
                     "nine",];
    let number_map = [("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)]
        .iter().cloned().collect::<std::collections::HashMap<_, _>>();

    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for result_line in reader.lines() {
        let line = result_line?;
        let mut sub_string = String::new();
        let mut rev_sub_string = String::new();
        for c in line.chars() {
            if c.is_numeric() {
                sum += c.to_digit(10).unwrap() * 10;
                break;
            }
            else if c.is_alphabetic() {
                sub_string.push(c);
                if num_words.iter().any(|word| sub_string.contains(word)) {
                    for (key, &value) in number_map.iter() {
                        if sub_string.contains(key) {
                            sum += value * 10;
                        }
                    }
                    break;
                }
            }

        }
        
        for c in line.chars().rev() {
            if c.is_numeric() {
                sum += c.to_digit(10).unwrap();
                break;
            }
            else if c.is_alphabetic() {
                rev_sub_string.insert(0, c);
                if num_words.iter().any(|word| rev_sub_string.contains(word)) {
                    for (key, &value) in number_map.iter() {
                        if rev_sub_string.contains(key) {
                            sum += value;
                        }
                    }
                    break;
                }
            }

        }
    }
    println!("Result is {}", sum);
    Ok(())
}