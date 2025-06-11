use std::fs;

fn main() {
    let contents = fs::read_to_string("directions.txt")
        .expect("Should have been able to read the file");

    let mut coordinates: Vec<(i32, i32)> = Vec::new(); 
    let mut position: (i32, i32) = (0, 0);
    let mut counter: u32 = 0;
    let mut index: u32 = 0;

    for c in contents.chars() {

        if c == '<' {
           position.0 -= 1
        }

        if c == '^' {
           position.1 += 1 
        }

        if c == 'v' {
           position.1 -= 1  
        }
        
        if c == '>' {
           position.0 += 1 
        }

        coordinates.push(position);
        println!("({};{})", position.0, position.1);

        for c in &coordinates {
    
            if c == &position {
                index += 1
            }
        }

        if index == 1 {
            counter += 1
        }

        index = 0;
    }

    println!("{}", counter)
}
