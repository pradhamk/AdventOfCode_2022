use std::{fs, path::Path};

fn main() {
    //Part One
    let mut calorie_list: Vec<i64> = vec![];

    let file_data = fs::read_to_string(Path::new("input.txt")).expect("Couldn't read input file");
    let nums_strings: Vec<&str> = file_data.split("\n\n").collect();

    for num_string in nums_strings {
        let mut calorie_count: i64 = 0;
        let calorie_strings: Vec<&str> = num_string.split("\n").collect();
        for calorie in calorie_strings {
            calorie_count += calorie.parse::<i64>().expect("Couldn't parse number");
        }

        calorie_list.push(calorie_count);
    }

    calorie_list.sort();
    calorie_list.reverse();

    let mut highest_ccount: i64 = 0;
    for calorie_count in &calorie_list {
        if calorie_count > &highest_ccount {
            highest_ccount = *calorie_count;
        }
    }

    println!("Highest Calorie Count: {} calories", highest_ccount);

    //Part Two
    println!("Total Calories of Top 3 Elves: {} calories", calorie_list[0] + calorie_list[1] + calorie_list[2]);

}
