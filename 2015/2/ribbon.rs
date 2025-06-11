use std::fs;

fn main() {
    println!("In file presents.txt");

    let contents = fs::read_to_string("presents.txt")
        .expect("Should have been able to read the file");

    let mut total_length: u32 = 0;

    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let dimensions: Result<Vec<u32>, _> = line
            .split('x')
            .map(|s| s.trim().parse())
            .collect();

        match dimensions {
            Ok(dims) if dims.len() == 3 => {
                let (l, w, h) = (dims[0], dims[1], dims[2]);

                let sorted_dimensions = sort_nums(&vec![l, w, h]);
                let ribbon = ribbon_length(sorted_dimensions[0], sorted_dimensions[1]);
                let bow = bow_length(sorted_dimensions[0], sorted_dimensions[1], sorted_dimensions[2]);

                total_length += ribbon + bow;
            }
            _ => {
                eprintln!("Skipping invalid line: {}", line);
                continue;
            }
        }
    }

    println!("{}", total_length)
}

fn ribbon_length(l: u32, w: u32) -> u32 {
    2 * (l + w)
}

fn bow_length(l: u32, w: u32, h: u32) -> u32 {
    l * w * h
}

fn sort_nums(numbers: &Vec<u32>) -> Vec<u32> {
    let mut sorted = numbers.clone();
    for _ in 0..sorted.len() {
        for i in 1..sorted.len() {
            if sorted[i] < sorted[i - 1] {
                sorted.swap(i - 1, i);
            }
        }
    }
    sorted
}
