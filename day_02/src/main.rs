use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let _ = day_02_part1("input.txt");
    let _ = day_02_part2("input.txt");
}

pub fn day_02_part1(filepath: &str) -> std::io::Result<()>{
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let num_of_red_cubes = 12;
    let num_of_green_cubes = 13;
    let num_of_blue_cubes = 14;
    let mut game_sum = 0;

    for result_line in reader.lines() {
        let line = result_line?;

        let colon_parts = line.split(":").collect::<Vec<&str>>();
        let id = colon_parts.get(0).unwrap().split(" ").collect::<Vec<&str>>();
        let turns_split_by_semicolon = colon_parts.get(1).unwrap().split(";").collect::<Vec<&str>>();
        let mut add_to_sum = true;

        for i in 0..turns_split_by_semicolon.len() {
            let results_split_by_comma = turns_split_by_semicolon.get(i).unwrap().split(",").collect::<Vec<&str>>();
            for result in results_split_by_comma {
                let split_result = result.trim().split(" ").collect::<Vec<&str>>();
                let cubes_num = split_result.get(0).unwrap().parse::<i32>().unwrap();
                
                if *split_result.get(1).unwrap() == "red" && cubes_num > num_of_red_cubes {
                    add_to_sum = false;
                }
                else if *split_result.get(1).unwrap() == "green" && cubes_num > num_of_green_cubes {
                    add_to_sum = false;
                }
                else if *split_result.get(1).unwrap() == "blue" && cubes_num > num_of_blue_cubes {
                    add_to_sum = false;
                }
            }
        }
        if add_to_sum {
            game_sum += id.get(1).unwrap().parse::<i32>().unwrap();
        }
    }
    println!("Result is {}", game_sum);
    Ok(())
}

pub fn day_02_part2(filepath: &str) -> std::io::Result<()>{
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut sum_min_products = 0;

    for result_line in reader.lines() {
        let line = result_line?;

        let mut min_num_of_red_cubes = 1;
        let mut min_num_of_green_cubes = 1;
        let mut min_num_of_blue_cubes = 1;

        let colon_parts = line.split(":").collect::<Vec<&str>>();
        let turns_split_by_semicolon = colon_parts.get(1).unwrap().split(";").collect::<Vec<&str>>();

        for i in 0..turns_split_by_semicolon.len() {
            let results_split_by_comma = turns_split_by_semicolon.get(i).unwrap().split(",").collect::<Vec<&str>>();
            for result in results_split_by_comma {
                let split_result = result.trim().split(" ").collect::<Vec<&str>>();
                let cubes_num = split_result.get(0).unwrap().parse::<i32>().unwrap();
                
                if *split_result.get(1).unwrap() == "red" && cubes_num > min_num_of_red_cubes {
                    min_num_of_red_cubes = cubes_num;
                }
                else if *split_result.get(1).unwrap() == "green" && cubes_num > min_num_of_green_cubes {
                    min_num_of_green_cubes = cubes_num;
                }
                else if *split_result.get(1).unwrap() == "blue" && cubes_num > min_num_of_blue_cubes {
                    min_num_of_blue_cubes = cubes_num;
                }
            }
        }
        sum_min_products += min_num_of_red_cubes * min_num_of_green_cubes * min_num_of_blue_cubes;
    }
    println!("Result is {}", sum_min_products);
    Ok(())
}