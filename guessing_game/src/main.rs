use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("The secret number is: {}", secret_number);


    // This makes an infinite loop
    loop {
        println!("Please input your guess.");

        // let is how variables are initialized
        // mut means the variable is mutable (can be changed)
        let mut guess = String::new();

        // Reading in a line from the terminal
        // the .expect() is for handling potential failure
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // This guess variable is shadowing the previous guess variable
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        // When running and input a character/string Rust is panicking here instead
        // of printing "Please type a number!". THIS IS WHAT SHOULD HAPPEN

        // Printing a string and using a placeholder
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // Exit the infinite loop
                break;
            }
        }
    }
}
