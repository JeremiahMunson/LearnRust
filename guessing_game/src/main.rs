use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    // let is how variables are initialized
    // mut means the variable is mutable (can be changed)
    let mut guess = String::new();

    // Reading in a line from the terminal
    // the .expect() is for handling potential failure
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    // Printing a string and using a placeholder
    println!("You guessed: {}", guess);
}
