use std::fs;
use std::collections::HashSet;

fn main() {
   let contents = fs::read_to_string("directions.txt")
        .expect("Should have been able to read the file");

    let mut position_robo: (i32, i32) = (0, 0);  
    let mut position_santa: (i32, i32) = (0, 0);  
    let mut visited_robo: HashSet<(i32, i32)> = HashSet::new();  
    let mut visited_santa: HashSet<(i32, i32)> = HashSet::new();  
    let mut turn = false;  
  
    visited_robo.insert(position_robo);  
    visited_santa.insert(position_santa);  
  
    for c in contents.chars() {  
        let (current_position, current_visited) = if turn {  
            (&mut position_santa, &mut visited_santa)  
        } else {  
            (&mut position_robo, &mut visited_robo)  
        };  
      
        match c {  
            '<' => current_position.0 -= 1,  
            '>' => current_position.0 += 1,  
            '^' => current_position.1 += 1,  
            'v' => current_position.1 -= 1,  
            _ => {}  
        }  
      
        current_visited.insert(*current_position);  
      
        turn = !turn;  
    }  
  
    let total_unique_houses = visited_robo.union(&visited_santa).count();  
    println!("Robo visited {} houses", visited_robo.len());  
    println!("Santa visited {} houses", visited_santa.len());  
    println!("Total unique houses: {}", total_unique_houses);
}
