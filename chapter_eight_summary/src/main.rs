use std::io;
use rand::Rng;
use mylib::{stats, pig_latin};


fn main() {
    /*
        The first exercise is to take a vector of numbers and calculate the mean, median, and 
        mode of the vector. I will use a vector of randomly generated integers where the user
        inputs the number of elements in the vector, the lower bound of the random numbers, and
        the upper bound of the random numbers.
    */

    // Getting vector information from user
    // The number of elements should be type 'usize'
    let number_elements: usize = stats::get_user_vector_info("How many elements should the vector have:", Option::Some(1), Option::None) as usize;
    let mut lower_bound;
    let mut upper_bound;
    // This loop checks to make sure the lower bound is less than the upper bound
    loop {
        lower_bound = stats::get_user_vector_info("Lower bound of numbers:", Option::None, Option::None);
        upper_bound = stats::get_user_vector_info("Upper bound of numbers:", Option::None, Option::None);

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
    
    // sorting the random numbers and printing them
    numbers.sort();
    println!("Numbers: {:?}", numbers);






     /*
        The second exercise is to convert strings to pig latin. The first
        consonant of each word is moved to the end of the word and "ay" is
        added, so "first" becomes "irst-fay". Words tht start with a vowel
        have "hay" added to the end instead ("apple" becomes "apple-hay").
        Keep in mind the details about UTF-8 encoding.

        I'm not entirely sure how pig latin would work with languages other
        than English, so I'm just going to assume that it works the same
        but I'm not implementing the vowel case.
    */

    println!("\nEnter text to be transformed into pig latin:");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Failed to read line.");
    let text_in_pig_latin = pig_latin::to_pig_latin(&text);

    println!("Pig Latin: {}", text_in_pig_latin);
}