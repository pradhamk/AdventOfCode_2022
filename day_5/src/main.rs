fn main() {
    let file_data = include_str!("../input.txt");
    let lines: Vec<&str> = file_data.split("\n").collect();
    let mut inital_map: Vec<&str> = vec![];

    let mut instruction_lines = vec![];
    let mut i = 0;
    for line in lines.clone() {
        if line.len() == 0 {
            instruction_lines = lines[i+1..lines.len()].to_vec();
            break
        }
        inital_map.push(line);
        i += 1;
    }

    let mut crates: Vec<&str> = vec![];
    let mut i = 0;
    for line in inital_map {
        loop {
            if i + 3 > 35 {
                break
            }
            crates.push(&line[i..i+3]);
            i+=4;
        }
        i = 0;
        
    }

    let mut vec_map: Vec<Vec<&str>> = vec![];
    let mut j = 0;
    loop {
        if j > 8 {
            break
        }
        let mut i = j % 10;
        let mut temp_vec: Vec<&str> = vec![];
        loop {
            if i >= 72 {
                break
            }
            if crates[i].trim().len() != 0 {
                temp_vec.push(crates[i]);
            }
            i += 9;
        }
        vec_map.push(temp_vec);
        j += 1;
    }
    
    let mut instructions: Vec<(u32,u32,u32)> = vec![];

    for line in instruction_lines {
        let temp_vec: Vec<&str> = line.split(" ").collect();
        instructions.push((
            temp_vec[1].parse().expect("Couldn't parse #"), 
            temp_vec[3].parse().expect("Couldn't parse #"),
            temp_vec[5].parse().expect("Couldn't parse #")
        ));
    }

    /* 
        for instruction in instructions.clone() {
            let howmany = instruction.0;
            let origin = (instruction.1 - 1) as usize;
            let destination = (instruction.2 - 1) as usize;

            for _i in 0..howmany {
                let moved_crate = vec_map[origin].remove(0);
                vec_map[destination].insert(0, moved_crate);
            }
        }

        println!("Part One: {:#?}", vec_map);
    */

    for instruction in instructions {
        let howmany = instruction.0;
        let origin = (instruction.1 - 1) as usize;
        let destination = (instruction.2 - 1) as usize;

        let mut temp_crate_stack: Vec<&str> = vec![];
        for _i in 0..howmany {
            let moved_crate = vec_map[origin].remove(0);
            temp_crate_stack.push(moved_crate);
        }
        temp_crate_stack.append(&mut vec_map[destination]);
        vec_map[destination] = temp_crate_stack;
    }

    println!("Part Two: {:#?}", vec_map);
}