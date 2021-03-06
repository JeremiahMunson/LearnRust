use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);


    // This makes an infinite loop
    loop {
        println!("Please input your guess.");

        // let is how variables are initialized
        // mut means the variable is mutable (can be changed)
        let mut guess = String::new();

        // Reading in a line from the terminal
        // the .expect() is for handling potential failure
        io::stdin().read_line(&mut guess).expect("Failed to read line");


        // Not in book (chapter 2) but wanted to try adding it
        // The .trim() removes white spaces at the start and end of the string
        if guess.trim() == "quit" {
            println!("The secret number was {}. Better luck next time!", secret_number);
            break;
        }

        // This guess variable is shadowing the previous guess variable
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // The above is now a match expression. If the guess.trim().parse() works
        // Ok, then the number is assigned. If there is an Error, continue (start the loop again)

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
