use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // let _ = day_01_part1("demo_input_part1.txt");
    let _ = day_01_part2("demo_input_part2.txt");
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
                if let Some(index) = num_words.iter().position(|r| sub_string.contains(r)) {
                    let index: u32 = index as u32;
                    sum += (index + 1) * 10;
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
                if let Some(index) = num_words.iter().position(|r| rev_sub_string.contains(r)) {
                    let index: u32 = index as u32;
                    sum += index + 1;
                    break;
                }
            }
        }
    }
    println!("Result is {}", sum);
    Ok(())
}