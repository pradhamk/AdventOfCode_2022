use std::{fs, path::Path};

fn get_points(opponent: &str, user: &str) -> i32 {
    match opponent {
        "A" => { //If opponent chooses rock
            match user {
                "X" => {
                    1 + 3
                }
                "Y" => {
                    2 + 6
                }
                "Z" => {
                    3 + 0
                }
                _ => { 0 }
            }
        }
        "B" => { //If opponent chooses paper
            match user {
                "X" => {
                    1 + 0
                }
                "Y" => {
                    2 + 3
                }
                "Z" => {
                    3 + 6
                }
                _ => { 0 }
            }
        }
        "C" => { //If opponent chooses scissors
            match user {
                "X" => {
                    1 + 6
                }
                "Y" => {
                    2 + 0
                }
                "Z" => {
                    3 + 3
                }
                _ => { 0 }
            }
        }
        _ => { 0 }
    }
}

fn get_points_2(opponent: &str, event: &str) -> i32 {
    match event {
        "X" => {
            //Need to lose
            match opponent {
                "A" => { //Rock
                    3
                }
                "B" => { //Paper
                    1
                }
                "C" => { //Scissors
                    2
                }
                _ => { 0 }
            }
        }
        "Y" => {
            //Need to tie
            match opponent {
                "A" => {
                    1 + 3
                }
                "B" => {
                    2 + 3
                }
                "C" => {
                    3 + 3
                }
                _ => { 0 }
            }
        }
        "Z" => {
            //Need to win
            match opponent {
                "A" => {
                    2 + 6
                }
                "B" => {
                    3 + 6
                }
                "C" => {
                    1 + 6
                }
                _ => { 0 }
            }
        }
        _ => { 0 }
    }
}

fn main() {
    let file_data = fs::read_to_string(Path::new("input.txt")).expect("Couldn't read input file");
    let file_lines: Vec<&str> = file_data.split("\n").collect();
    
    let mut total_points = 0;
    let mut total_points_2 = 0;

    for line in file_lines {
        let data: Vec<&str> = line.split(" ").collect();
        total_points += get_points(data[0], data[1]);
        total_points_2 += get_points_2(data[0], data[1]);
    }

    println!("Total Score: {} points", total_points);
    
    println!("Total Score Part 2: {} points", total_points_2);
}