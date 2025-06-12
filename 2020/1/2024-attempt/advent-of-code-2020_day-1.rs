use std::fs;
use std::num::ParseIntError;

fn read_numbers_from_file(file_path: &str) -> Result<Vec<i32>, ParseIntError> {
    // Read the file contents into a string
    let contents = fs::read_to_string(file_path).expect("Failed to read the file");

    // Split the contents by whitespace and parse each number
    let numbers: Result<Vec<i32>, ParseIntError> = contents
        .split_whitespace() // Split by whitespace
        .map(str::parse)   // Parse each string to i32
        .collect();        // Collect into a Vec<i32>

    numbers 
}

fn find_matching_numbers(nums:&Vec<i32>) {
   for num1 in nums.iter() { 
       for num2 in nums.iter() {
           if num1 + num2 == 2020 {
                let result: i32 = num1 * num2;
                println!("{} * {} = {}", num1, num2, result);
                return;
           }
       }
   }
}

fn main() {
    let file_path = "advent-of-code-2020_day-1_data.txt";
    match read_numbers_from_file(file_path) {
        Ok(numbers) => {
            find_matching_numbers(&numbers);
        }
        Err(e) => {
            eprintln!("Error parsing numbers: {}", e);
        }
    }
}

