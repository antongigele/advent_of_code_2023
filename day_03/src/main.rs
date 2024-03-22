use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let _ = day_03_part1("demo_input.txt");
    // let _ = day_03_part2("demo_input.txt");
}

pub fn create_matrix(filepath: &str) -> std::io::Result<Vec<Vec<char>>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut char_matrix: Vec<Vec<char>> = Vec::new();
    for result_line in reader.lines() {
        let line = result_line?;
        
        let chars: Vec<char> = line.chars().collect();
        char_matrix.push(chars);
    }
    Ok(char_matrix)
}

pub fn day_03_part1(filepath: &str) -> std::io::Result<()>{
    let char_matrix = create_matrix(filepath).unwrap();

    let mut part_numbers_sum: u32 = 0;
    let mut intermediate_sum: u32 = 0;
    let mut base10_multiplier: u32 = 0;
    let mut neighbour_signs_container: Vec<char> = Vec::new();

    for i in 0..char_matrix.len() {
        println!("{:?}", char_matrix.get(i).unwrap());
        for j in (0..char_matrix.get(i).unwrap().len()).rev() {
            let entry: &char = char_matrix.get(i).unwrap().get(j).unwrap();
            if !entry.is_digit(10) {
                base10_multiplier = 0;
                intermediate_sum = 0;
                neighbour_signs_container.clear();
            }
            else if entry.is_digit(10) {
                intermediate_sum += entry.to_digit(10).unwrap() * 10_u32.pow(base10_multiplier);
                base10_multiplier += 1;
                // if another sign except from . is found, intermediate_sum gets added to the sum
                // else if the container contains only ., intermediate_sum is not added to the sum
            }
        }
    }
    println!("{:?}", part_numbers_sum);
    Ok(())
}   