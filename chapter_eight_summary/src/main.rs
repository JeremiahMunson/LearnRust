use std::io;
use rand::Rng;
use mylib::stats;

fn main() {
    /*
        The first exercise is to take a vector of numbers and calculate the mean, median, and 
        mode of the vector. I will use a vector of randomly generated integers where the user
        inputs the number of elements in the vector, the lower bound of the random numbers, and
        the upper bound of the random numbers.
    */

    // Getting vector information from user
    // The number of elements should be type 'usize'
    let number_elements: usize = get_user_vector_info("How many elements should the vector have:", Option::Some(1), Option::None) as usize;
    let mut lower_bound;
    let mut upper_bound;
    // This loop checks to make sure the lower bound is less than the upper bound
    loop {
        lower_bound = get_user_vector_info("Lower bound of numbers:", Option::None, Option::None);
        upper_bound = get_user_vector_info("Upper bound of numbers:", Option::None, Option::None);

        // the lower_bound should be less than or equal to the upper_bound
        if lower_bound <= upper_bound {
            break;
        }
    }

    // Creating the vector using user input
    let mut numbers: Vec<i32> = Vec::new();
    for _i in 0..number_elements {
        numbers.push(rand::thread_rng().gen_range(lower_bound, upper_bound+1));
    }

    // Calculate and print mean value of the vector
    let mean = stats::calc_mean(&numbers);
    println!("The mean value of the vector is {}", mean);

    // Calculate and print the median value of the vector
    let median = stats::calc_median(&numbers);
    median.print();

    // Calculate the mode(s) of the vector
    let modes = stats::calc_mode(&numbers);
    // Printing the mode(s) of the vector
    if modes.len() == 1 {
        println!("Mode is: {}", modes[0]);
    } else {
        // Formatting the modes string
        let mut to_print = String::from("Modes are:");
        for n in modes.iter() {
            to_print = format!("{} {},", to_print, n);
        }

        // Remove the last ','
        to_print.pop();
        println!("{}", to_print);
    }
    

    numbers.sort();
    println!("Numbers: {:?}", numbers);
}


fn get_user_vector_info(message: &str, lower_lim: Option<i32>, upper_lim: Option<i32>) -> i32{
    let mut user_input: String;
    loop {
        println!("{}", message);
        user_input = String::new();

        io::stdin().read_line(&mut user_input).expect("Failed to read line.");

        let var: i32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // If lower_lim is not None, make sure the input is greater than or equal to the lower limit
        if let Some(i) = lower_lim {
            if i > var {
                continue;
            }
        }

        // If upper_lim is not None, make sure the input is less than or equal to the upper limit
        if let Some(i) = upper_lim {
            if i < var {
                continue;
            }
        }

        return var;
    }
}