fn main() {
    let file_data = include_str!("../input.txt");
    let split_file: Vec<&str> = file_data.split("\n").collect();

    let mut sacks: Vec<(&str, &str)> = vec![];

    for sack in split_file {
        sacks.push((&sack[0..sack.len()/2], &sack[sack.len()/2..sack.len()]));
    }

    let mut sum = 0;

    for sack in sacks.clone() {
        for char in sack.0.chars() {
            if sack.1.contains(char) {
                let ascii_code = char as u8;
                if ascii_code.is_ascii_lowercase() {
                    sum += (ascii_code - 96) as i64;
                } else {
                    sum += (ascii_code - 38) as i64;
                }
                break
            }
        }
    }

    println!("Part One Sum: {}", sum);

    let groups: Vec<&[(&str, &str)]> = sacks.chunks(3).collect();

    let mut sum_2 = 0;
    for group in groups {
        for char in (group[0].0.to_owned() + group[0].1).chars() {
            if (group[1].0.to_owned() + group[1].1).contains(char) && (group[2].0.to_owned() + group[2].1).contains(char) {
                let ascii_code = char as u8;
                if ascii_code.is_ascii_lowercase() {
                    sum_2 += (ascii_code - 96) as i64;
                } else {
                    sum_2 += (ascii_code - 38) as i64;
                }
                break
            }
        }
    }

    println!("Part Two Sum: {}", sum_2);
}
