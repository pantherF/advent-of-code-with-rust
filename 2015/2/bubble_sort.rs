fn main() {
    let mut vector: Vec<u32> = vec![34, 5, 7, 2, 1];
    let test = sort_nums(&mut vector);
    println!("{:?}", test);
}

fn sort_nums(numbers: &mut Vec<u32>) -> &Vec<u32> {
    for _ in 0..numbers.len() {
        for i in 1..numbers.len() {
            if numbers[i] < numbers[i - 1] {
                numbers.swap(i - 1, i);
            }
        }
    }

    numbers
}
