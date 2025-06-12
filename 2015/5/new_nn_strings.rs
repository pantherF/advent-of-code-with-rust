use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let mut counter = 0u32;
    let mut n_counter = 0u32;

    for line in contents.lines() {
    
        if (rule_two(&line.to_string()) == true) && (rule_one(&line.to_string()) == true) {
            counter += 1;
            println!("nice: {}", line);
        }
        
        n_counter += 1;
        println!("naughty: {}", line);
    }

    println!("total: {}", contents.lines().count());
    println!("nice: {}", counter);
    println!("naughty: {}", n_counter);
}

fn rule_one(line: &String) -> bool {
    let chars: Vec<char> = line.chars().collect();
    let mut doubles: Vec<(char, char, u8, u8)> = Vec::new();

    for i in 1..chars.len() {
        let c1 = chars[i-1];
        let c2 = chars[i];
        let i1: u8 = i as u8 - 1;
        let i2: u8 = i as u8;

        doubles.push((c1, c2, i1, i2));
    }

    for (i, entry1) in doubles.iter().enumerate() {
        for entry2 in doubles.iter().skip(i + 1) {
            if same_letter_combo(entry1, entry2) == true && index_match(entry1, entry2) == false {
                return true;
                //println!("{:?}, {:?}", entry1, entry2);
            }
        }
    }

    false
}

fn same_letter_combo(tuple1: &(char, char, u8, u8), tuple2: &(char, char, u8, u8)) -> bool {
    tuple1.0 == tuple2.0 && tuple1.1 == tuple2.1
}

fn index_match(tuple1: &(char, char, u8, u8), tuple2: &(char, char, u8, u8)) -> bool {
    if tuple1.2 == tuple2.2 || tuple1.2 == tuple2.3 {
        return true;
    }
    else if tuple1.3 == tuple2.2 || tuple1.3 == tuple2.3 {
        return true;
    }

    false
}

fn rule_two(line: &String) -> bool {
    let chars: Vec<char> = line.chars().collect();

    for i in 1..chars.len()-1 {
        let c1 = chars[i-1];
        let c2 = chars[i];
        let c3 = chars[i+1];

        if c1 == c3 {
            //println!("{}, {}, {}", c1, c2, c3);
            return true;
        }
    }

    false
}
