use md5;

fn main() {
    let secret_key = "ckczppom";
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
