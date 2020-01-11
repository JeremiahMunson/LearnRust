use std::io;
use rand::Rng;
use std::collections::HashMap;

enum Median {
    None,
    One(i32),
    Two(i32,i32),
}

fn main() {
    /*
        The first exercise is to take a vector of numbers and calculate the mean, median, and 
        mode of the vector. I will use a vector of randomly generated integers where the user
        inputs the number of elements in the vector, the lower bound of the random numbers, and
        the upper bound of the random numbers.
    */

    // Getting vector information from user
    let number_elements: usize = get_user_vector_info("How many elements should the vector have:", Option::Some(1), Option::None) as usize;
    let mut lower_bound;
    let mut upper_bound;
    // This loop checks to make sure the lower bound is less than the upper bound
    loop {
        lower_bound = get_user_vector_info("Lower bound of numbers:", Option::None, Option::None);
        upper_bound = get_user_vector_info("Upper bound of numbers:", Option::None, Option::None);

        if lower_bound <= upper_bound {
            break;
        }

        println!("Upper bound must be greater than or equal to lower bound");
    }

    // Creating the vector using user input and making hash map with frequency of numbers
    let mut numbers: Vec<i32> = Vec::new();
    let mut number_frequency = HashMap::new();
    for _i in 0..number_elements {
        let random_number = rand::thread_rng().gen_range(lower_bound, upper_bound+1);
        numbers.push(random_number);

        let count = number_frequency.entry(random_number).or_insert(0);
        *count += 1;
    }

    // CALCULATING THE MEAN
    let mut sum: i32 = 0;
    for arr in numbers.iter() {
        sum += arr;
    }
    let average = sum as f64 / number_elements as f64;
    println!("The mean value of the vector is {}", average);


    // Sort the numbers from lowest to highest
    numbers.sort_by(|a, b| a.cmp(b));

    // The median may not be one number, it could be two
    let index = number_elements/2;
    let mut median = Median::None;
    if number_elements%2 == 1 {
        if let Some(i) = numbers.get(index) {
            median = Median::One(*i);
        }
    } else {
        let first_index = index - 1;

        // Get 'i' and 'j', the two elements for the median and check if
        // they are the same. If same, median is just that number. If
        // different, both numbers represent the median
        if let Some(i) = numbers.get(first_index) {
            if let Some(j) = numbers.get(index) {
                median = if i == j {
                    Median::One(*i)
                } else {
                    Median::Two(*i,*j)
                };
            }
        }
    };

    match median {
        Median::None => println!("There is no median somehow..."),
        Median::One(i) => println!("Median value is: {}", i),
        Median::Two(i,j) => println!("Median value is: {} and {}", i, j),
    };


    let mut max_freq = 0;
    let mut nums = Vec::new();
    for (num, freq) in number_frequency.iter() {
        if *freq > max_freq {
            nums.clear();
            nums.push(num);
            max_freq = *freq;
        } else if *freq == max_freq {
            nums.push(num);
        }
    }
    nums.sort_by(|a,b| a.cmp(b));
    if nums.len() == 1 {
        println!("Mode is: {}", nums[0]);
    } else {
        let mut to_print = String::from("Modes are:");
        for n in nums.iter() {
            to_print = format!("{} {},", to_print, n);
        }
        to_print.pop();
        println!("{}", to_print);
    }
    //println!("Number of modes: {}", nums.len());
    


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