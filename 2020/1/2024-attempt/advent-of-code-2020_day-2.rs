mod file_reader; 
use file_reader::{read_data_from_file, Data}; 

fn extract_numbers_from_text(text: &str) -> Option<(i32, i32)> {
    let parts: Vec<&str> = text.split('-').collect();

    if parts.len() == 2 {
        if let (Ok(first), Ok(second)) = (parts[0].trim().parse::<i32>(), parts[1].trim().parse::<i32>()) {
            return Some((first, second));
        }
    }
    
    None
}


fn validate_password(data: (&Data, &Data, &Data)) -> bool {
    let range = data.0;
    let letter_data = data.1;
    let string = data.2;

    let mut from: i32 = 0;
    let mut to: i32 = 0;

    match range {
        Data::Text(numbers) => {
            if let Some((num1, num2)) = extract_numbers_from_text(numbers) {
                from = num1;
                to = num2;
            } else {
                println!("Failed to extract numbers from '{}'", numbers);
            }
        }
        _ => {
            println!("The first item is not a text value: {:?}", range);
            return false;
        }
    }

    let letter = match letter_data {
        Data::Text(ref text) => text.chars().next().unwrap_or(' '),
        _ => {
            println!("The second item is not a text value: {:?}", letter_data);
            return false;
        }
    };

    let string_to_search = match string {
        Data::Text(ref text) => text,
        _ => {
            println!("The third item is not a text value: {:?}", string);
            return false;
        }
    };

    let count = string_to_search.chars().filter(|&c| c == letter).count() as i32;

    if count >= from && count <= to {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let file_path = "advent-of-code-2020_day-2_data.txt";
    let mut valid_counter = 0;

    match read_data_from_file(file_path) {
        Ok(data) => {
            for i in (0..data.len()).step_by(3) {
                if i + 2 < data.len() {
                    let temp = (
                        &data[i],
                        &data[i + 1],
                        &data[i + 2],
                    );
                    if validate_password(temp) == true {
                        valid_counter += 1;
                    }
                } else {
                    println!("Not enough elements to form a complete set at index {}", i);
                }
            } 
        }
        Err(e) => {
            eprintln!("Error parsing data: {}", e);
        }
    }

    println!("Valid passwords: {}", valid_counter);
}

