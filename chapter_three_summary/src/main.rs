use std::io;

fn main() {

    let mut keep_going = true;

    println!("\nFahrenheit to Celsius");
    loop {
        let mut user_input = String::new();
        println!("Input temperature in fahrenheit:");
        io::stdin().read_line(&mut user_input).expect("Failed to read line.");

        let trimmed = user_input.trim();

        if trimmed == "next" || trimmed == "continue" {
            break;
        } else if trimmed == "exit" || trimmed == "quit" {
            keep_going = false;
            break;
        }

        let fahrenheit: f64 = match trimmed.parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        // I have gone through this before and know about ownership
        // see chapter 4 to understand why it's &fahrenheit not fahrenheit
        let celsius = fahrenheit_to_celsius(&fahrenheit);
        println!("Temperature in celsisus: {} degrees", celsius);
    }

    if keep_going {
        println!("\nCelsius to Fahrenheit");
        loop {
            let mut user_input = String::new();
            println!("Input temperature in celsius:");
            io::stdin().read_line(&mut user_input).expect("Failed to read line.");

            let trimmed = user_input.trim();

            if trimmed == "next" || trimmed == "continue" {
                break;
            } else if trimmed == "exit" || trimmed == "quit" {
                keep_going = false;
                break;
            }

            let celsius: f64 = match trimmed.parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            // I have gone through this before and know about ownership
            // see chapter 4 to understand why it's &celsius not celsius
            let fahrenheit = celsius_to_fahrenheit(&celsius);
            println!("Temperature in fahrenheit: {} degrees", fahrenheit);
        }
    }

    if keep_going {
        println!("\nFind the nth Fibonacci number");
        loop {
            let mut user_input = String::new();
            println!("Input which fibonacci number you'd like (what n):");

            io::stdin().read_line(&mut user_input).expect("Failed to read line.");

            let trimmed = user_input.trim();

            if trimmed == "next" || trimmed == "continue" {
                break;
            } else if trimmed == "exit" || trimmed == "quit" {
                keep_going = false;
                break;
            }

            let n: u32 = match trimmed.parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            let fib = nth_fibonacci_number(n);
            println!("That Fibonacci number is: {}", fib);
        }
    }

    if keep_going {
        println!("\nThe 12 days of Christmas");
        loop {
            let mut user_input = String::new();

            println!("Input the day you would like to know what gifts my true love gave to me:");

            io::stdin().read_line(&mut user_input).expect("Failed to read line.");

            let trimmed = user_input.trim();

            if trimmed == "next" || trimmed == "continue" {
                break;
            } else if trimmed == "exit" || trimmed == "quit" {
                keep_going = false;
                break;
            }

            let day = match trimmed.parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            twelve_days_gifts(&day);
        }
    }

    if keep_going {
        println!("\nCummulative Presents From 12 Days of Christmas");
        loop {
            let mut user_input = String::new();

            println!("Please input the day of Christmas to see cummulative presents up to then:");

            io::stdin().read_line(&mut user_input).expect("Failed to read line.");

            let trimmed = user_input.trim();

            if trimmed == "next" || trimmed == "continue" {
                break;
            } else if trimmed == "exit" || trimmed == "quit" {
                break;
            }

            let day = match trimmed.parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            cummulative_twelve_days(&day);
        }
    }
}

fn fahrenheit_to_celsius(temp: &f64) -> f64 {
    // f = (c-32.0) 9.0/5.0
    (temp-32.0)*5.0/9.0
}

fn celsius_to_fahrenheit(temp: &f64) -> f64 {
    // c = 9.0/5.0 f + 32.0
    temp*9.0/5.0 + 32.0
}

// fibonacci numbers are always positive
// they also grow very quickly (beyond n=12 it is larger than n^2)
fn nth_fibonacci_number(n: u32) -> u128 {
    // I'll start with n == 0, the fibonacci number is 0
    // When n == 1 and when n == 2, the fibonacci number is 1


    // This is backward at first so for n = 0 it doesn't go through
    //  the loop at all so fib = 0.
    // Then for n = 1 it goes through the loop once so it adds 1 to fib so
    //  fib = 1 and previous is 1-1 = 0 which is correct.
    let mut fib = 0;
    let mut previous = 1;
    for _x in 0..n {
        fib += previous;
        previous = fib-previous;
    }
    fib
}

fn twelve_days_gifts(day: &usize) {

    // The day must be between 1 and 12 inclusive
    // Type is unsigned so no worry over negative
    if *day == 0 || *day > 12 {
        return;
    }

    let gifts = [
        "A partridge in a pear tree!",
        "Two turtle doves and",
        "Three french hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a laying",
        "Seven swans a swimming",
        "Eight maids a milking",
        "Nine ladies dancing",
        "Ten lords a leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    if *day == 1 {
        println!("On the 1st day of Christmas my true love gave to me...");
    } else if *day == 2 {
        println!("On the 2nd day of Christmas my true love gave to me...");
    } else if *day == 3 {
        println!("On the 3rd day of Christmas my true love gave to me...");
    } else {
        println!("On the {}th day of Christmas my true love gave to me...", day);
    }

    for number in (0..*day).rev() {
        println!("{}", gifts[number]);
    }
}

fn cummulative_twelve_days(day: &usize) {
    // The day must be between 1 and 12 inclusive
    // Type is unsigned so no worry over negative
    if *day == 0 || *day > 12 {
        return;
    }

    if *day == 1 {
        println!("On the 1st day of Christmas my true love had given me...\n1 patridge in a pear tree!");
        return;
    } else if *day == 2 {
        println!("On the 2nd day of Christmas my true love had given me...");
    } else if *day == 3 {
        println!("On the 3rd day of Christmas my true love had given me...");
    } else {
        println!("On the {}th day of Christmas my true love had given me...", day);
    }

    let gifts = [
        "partridges in a pear tree!",
        "turtle doves and",
        "french hens",
        "calling birds",
        "gold rings",
        "geese a laying",
        "swans a swimming",
        "maids a milking",
        "ladies dancing",
        "lords a leaping",
        "pipers piping",
        "drummers drumming"
    ];

    let mut number_presents;
    for i in (0..*day).rev() {
        number_presents = (i+1)*(*day - i);
        println!("{} {}", number_presents, gifts[i]);
    }

}