use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("src/input.txt")?;
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