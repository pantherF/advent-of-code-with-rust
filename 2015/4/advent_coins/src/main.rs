use md5;  
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};  
use std::sync::Arc;  
use std::thread;

fn main() {
    let secret_key = "ckczppom";  
    five_zeroes(secret_key.to_string());
    six_zeroes(secret_key.to_string());
}

fn six_zeroes(secret_key: String) {
    let num_threads = 4;  
    let found = Arc::new(AtomicBool::new(false));  
    let result = Arc::new(AtomicU32::new(0));  
    let mut handles = vec![];  
  
    for thread_id in 0..num_threads {  
        let secret_key = secret_key.to_string();  
        let found = Arc::clone(&found);  
        let result = Arc::clone(&result);  
          
        let handle = thread::spawn(move || {  
            let mut number = thread_id as u32;  
              
            while !found.load(Ordering::Relaxed) {  
                let input = format!("{}{}", secret_key, number);  
                let digest = md5::compute(input);  
                  
                if digest[0] == 0 && digest[1] == 0 && digest[2] == 0 {  
                    found.store(true, Ordering::Relaxed);  
                    result.store(number, Ordering::Relaxed);  
                    println!("Thread {} found: {}", thread_id, number);  
                    break;  
                }  
                  
                number += num_threads as u32;  
            }  
        });  
          
        handles.push(handle);  
    }  
      
    for handle in handles {  
        handle.join().unwrap();  
    }  
      
    let final_number = result.load(Ordering::Relaxed);  
    let input = format!("{}{}", secret_key, final_number);  
    let digest = md5::compute(input);  
    println!("Final result: {} -> {:032x}", final_number, digest);
}
fn five_zeroes(secret_key: String) {
    let mut brk: bool = false;
    let mut number: u32 = 0;

    while brk == false {

        let input = secret_key.to_owned() + &number.to_string();

        let digest = md5::compute(input);
        let hex_hash = format!("{:x}", digest);

        let first_five: String = hex_hash.chars().take(5).collect();

        if first_five == "00000" {
            println!("{:x}", digest);
            println!("{}", number);
            brk = true;
        }

        number += 1;
    }
}
