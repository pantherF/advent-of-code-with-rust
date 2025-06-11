use std::fs;

fn main() {
    println!("In file presents.txt");

    let contents = fs::read_to_string("presents.txt")
        .expect("Should have been able to read the file");

    let mut total_square_feet: u32 = 0;

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
                let surface_area_with_slack = surface_area_with_slack(l, w, h);

                total_square_feet += surface_area_with_slack;
            }
            _ => {
                eprintln!("Skipping invalid line: {}", line);
                continue;
            }
        }
    }

    println!("{}", total_square_feet)
}

fn surface_area_with_slack(l: u32, w: u32, h: u32) -> u32 {
    let lw = l*w;
    let wh = w*h;
    let hl = h*l;

    let areas: Vec<u32> = vec![lw, wh, hl];
    let smallest_area: u32 = min_num(&areas);

    2 * (lw + wh + hl) + smallest_area
}

fn min_num(numbers: &Vec<u32>) -> u32 {  
    let mut smallest = numbers[0];  
  
    for item in numbers.iter() {  
        if *item < smallest {  
            smallest = *item;  
        }  
    }  
  
    smallest  
}
