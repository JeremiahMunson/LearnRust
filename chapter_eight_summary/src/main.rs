use std::io;
use rand::Rng;

fn main() {
    let mut number_elements: i32;
    loop {
        println!("What size would vector would you like?");
        let mut user_size = String::new();
        io::stdin().read_line(&mut user_size).expect("Failed to read line.");
        number_elements = match user_size.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break;
    }
        
    println!("Lower bound of numbers");
    let mut user_lower_bound = String::new();
    io::stdin().read_line(&mut user_lower_bound).expect("Failed to read line.");

    println!("Upper bound of numbers");
    let mut user_upper_bound = String::new();
    io::stdin().read_line(&mut user_upper_bound).expect("Failed to read line.");

    println!("User wants a vector with {} elements with values ranging from {} to {}", number_elements, user_lower_bound, user_upper_bound);

   
}