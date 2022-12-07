fn solve(input: &str, one: bool) {
    let mut chunk: Vec<char> = vec![];
    let mut marker = 0;

    let start_limit: i32;
    if one { start_limit = 4 }
    else { start_limit = 14 } 

    for char in input.chars() {
        if marker < start_limit {
            chunk.push(char);
            marker += 1;
            continue
        }

        let mut temp_chunk = chunk.clone();
        temp_chunk.sort();
        let mut old_char: char = 'a'; //random char
        let mut dup_count = 0;
        for val in temp_chunk {
            if old_char == val {
                dup_count += 1;
                break
            }
            old_char = val;
        }

        if dup_count == 0 {
            break
        }

        chunk.remove(0);
        chunk.push(char);
    
        marker += 1;
    }

    if one {
        println!("Part One Answer: {}", marker);
    } else {
        println!("Part Two Answer: {}", marker);
    }
}


fn main() {
    let input = include_str!("../input.txt");
    solve(input, true);
    solve(input, false);
}