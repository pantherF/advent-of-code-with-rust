use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let rule_three = ["ab", "cd", "pq", "xy"];

    let mut counter = 0u32;
    let mut n_counter = 0u32;

    for line in contents.lines() {
    
        if rule_three.iter().any(|&sub| line.contains(sub)){
            println!("naughty: {}", line);
            n_counter += 1;
            continue;
        }

        if (rule_two(&line.to_string()) == true) && (rule_one(&line.to_string()) == true) {
            counter += 1;
            println!("nice: {}", line);
        }
        else {
            n_counter += 1;
            println!("naughty: {}", line);
        }
    }

    println!("total: {}", contents.lines().count());
    println!("nice: {}", counter);
    println!("naughty: {}", n_counter);
}

fn rule_one(line: &String) -> bool {  
    let chars: Vec<char> = line.chars().collect();  
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut counter = 0u32;
    
    for i in 0..chars.len() {
        for j in 0..vowels.len(){
            if chars[i] == vowels[j] {
                counter += 1;  
            }   
        }  
    }  
    
    if counter >= 3 {
        return true;
    }
    
    false
}

fn rule_two(line: &String) -> bool {
    let chars: Vec<char> = line.chars().collect();

    for i in 1..chars.len() {
        if chars[i] == chars[i-1] {
            return true;
        }
    }

    false
}
