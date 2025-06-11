use std::fs;

fn main() {
    let contents = fs::read_to_string("floor-instructions.txt")
        .expect("Should have been able to read the file");

    let mut counter: i32 = 0; 
    let mut position: i32 = 0;

    for c in contents.chars() {
        position += 1;

        if c == '(' {
            counter += 1;
        }

        if c == ')' {
            counter -= 1;
        }

        if counter == -1 {
            println!("{}", position)
        }
    }

    println!("{}", counter)
}
