fn main() {
    let file_data = include_str!("../input.txt");
    let lines: Vec<&str> = file_data.split("\n").collect();

    let mut total_count = 0;
    let mut total_count_2 = 0;
    for line in lines {
        let elves: Vec<&str> = line.split(",").collect();
        let mut ranges: Vec<(i32, i32)> = vec![];

        for elf in elves {
            let range: Vec<&str> = elf.split("-").collect();
            ranges.push((
                range[0].parse().expect("Couldn't parse number"),
                range[1].parse().expect("Couldn't parse number")
            ));
        }

        if (ranges[1].0 >= ranges[0].0 && ranges[1].1 <= ranges[0].1) 
        || (ranges[0].0 >= ranges[1].0 && ranges[0].1 <= ranges[1].1) 
        {
            total_count += 1;
        }

        let range1: Vec<i32> = (ranges[0].0..=ranges[0].1).collect();
        let range2: Vec<i32> = (ranges[1].0..=ranges[1].1).collect();

        for num in range1 {
            if range2.contains(&num) {
                total_count_2 += 1;
                break
            }
        }
    }

    println!("Part One Sum: {}", total_count);

    println!("Part Two Sum: {}", total_count_2);
}
