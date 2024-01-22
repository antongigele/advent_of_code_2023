use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let num_words = ["one", 
                     "two", 
                     "three", 
                     "four",
                     "five",
                     "six",
                     "seven",
                     "eight",
                     "nine",];
    let first = num_words[0];
    println!("Result is {}", first);
    """
    loop through each line
    loop through each char
    from the left, check if one of the num_words is a substring and take the first one
    form the right, check if one of the num_words is a substring and take the first one
    convert the num_words as numbers and do the same as in day_01_a
    """
    // let file = File::open("src/input.txt")?;
    // let reader = BufReader::new(file);

    // let mut sum = 0;
    // for result_line in reader.lines() {
    //     let line = result_line?;
    //     for c in line.chars() {
    //         if c.is_numeric() {
    //             sum += c.to_digit(10).unwrap() * 10;
    //             break;
    //         }
    //     }
    //     for c in line.chars().rev() {
    //         if c.is_numeric() {
    //             sum += c.to_digit(10).unwrap();
    //             break;
    //         }
    //     }
    // }
    // println!("Result is {}", sum);
    Ok(())
    
}
