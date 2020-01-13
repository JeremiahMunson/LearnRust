pub mod stats {
    pub use std::io;

    pub enum Median {
        None,
        One(i32),
        Two(i32,i32),
    }

    impl Median {
        pub fn print(&self) {
            match self {
                Median::None => println!("There is no median."),
                Median::One(i) => println!("Median value is: {}", i),
                Median::Two(i,j) => println!("Median values are: {} and {}", i, j),
            };
        }
    }

    pub fn get_user_vector_info(message: &str, lower_lim: Option<i32>, upper_lim: Option<i32>) -> i32{
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

    pub fn calc_mean(v: &Vec<i32>) -> f64 {
        // CALCULATING THE MEAN
        let mut sum: i32 = 0;
        for arr in v.iter() {
            sum += arr;
        }
        
        // Average should be a float, not an integer
        sum as f64 / v.len() as f64
    }

    pub fn calc_median(v: &Vec<i32>) -> Median {
        let number_elements = v.len();
        let mut numbers = v.clone();
    
        // Sort the numbers from lowest to highest
        //numbers.sort_by(|a, b| a.cmp(b));
        numbers.sort();

        let index = number_elements/2;
        let mut median = Median::None;


        // If number_elements % 2 == 1 there is always 1 median, otherwise 
        // there could be 2 values
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

        median
    }

    pub fn calc_mode(v: &Vec<i32>) -> Vec<i32> {
        
        let mut number_frequency = std::collections::HashMap::new();
        // Loop through numbers and update frequency hash map
        // Double dereference because reference to vector which references the data
        for num in v.iter() {
            let count = number_frequency.entry(num).or_insert(0);
            *count += 1;
        }

        let mut max_freq = 0;
        let mut modes = Vec::new();
        for (num, freq) in number_frequency.iter() {
            if *freq > max_freq {
                modes.clear();
                
                modes.push(**num);
                max_freq = *freq;
            } else if *freq == max_freq {
                modes.push(**num);
            }
        }
        //modes.sort_by(|a,b| a.cmp(b));
        modes.sort();

        modes
    }
}


pub mod pig_latin {
    use unicode_segmentation::UnicodeSegmentation;

    pub fn to_pig_latin(txt: &str) -> String {
        //let mut return_string = String::from("Text in pig latin:");
        let mut return_string = String::new();

        // Split text into different words, then go through each word making it pig latin
        let words = txt.split_whitespace();
        for slice in words {
            // This gets the word (slice) as a vector of graphemes to handle any language
            let mut graphemes = UnicodeSegmentation::graphemes(slice, true).collect::<Vec<&str>>();

            // Check if word starts with a vowel and add "h" to the start of the word if it does
            let first = graphemes.get(0);
            //println!("{:?}", first);
            match first {
                Some(s) => {
                    // Would like to include vowels with accents at some point but their are a lot of them and 
                    // I'd rather keep learning Rust than adding additional cases.
                    if *s == "a" || *s == "e" || *s == "i" || *s == "o" || *s == "u" || *s == "A" || *s == "E" || *s == "I" || *s == "O" || *s == "U" {
                        graphemes.insert(0,"h");
                    }
                },
                None => continue,
            };
            
            // Add "-" to the end, move the first letter to be the last, add "ay" to the end
            // We always want to move the first letter to the end because vowels had "h" added
            // to the front above
            graphemes.push("-");
            graphemes.rotate_left(1);
            graphemes.push("ay");

            // Add the word to the existing phrase
            return_string = format!("{} {}", return_string, graphemes.concat());
        }

        return_string
    }
}



pub mod department {
    use std::io;
    use std::collections::{LinkedList, HashMap};

    pub struct Directory {
        employees_by_department: HashMap<String, Vec<String>>,
        all_employees: Vec<String>,
    }

    // Looked ahead for this
    enum Return<T> {
        Some(T),
        Empty,
        None,
    }

    impl Directory {
        // Make new, empty directoy
        pub fn new() -> Directory {
            Directory {
                employees_by_department: HashMap::new(),
                all_employees: Vec::new(),
            }
        }

        pub fn update(&mut self) {
            let mut user_input;

            loop {
                user_input = String::new();
                io::stdin().read_line(&mut user_input).expect("Failed to read line.");
                let mut split_text = user_input.split_whitespace();
                match split_text.next() {
                    Some("Add") => {Directory::add_employee(self, &mut split_text);},
                    Some("Print") => {Directory::print(self, &mut split_text);},
                    _ => {println!("Else");},
                };
                break;
            }
        }

        fn print(&mut self, txt: &mut std::str::SplitWhitespace) {
            match Directory::get_name(txt, Option::None) {
                Return::Some(s) => Directory::print_department(self, &s),
                Return::Empty => println!("{:?}", self.all_employees),
                Return::None => return (),
            };
        }

        fn print_department(&mut self, dept: &str) {
            let employees = match self.employees_by_department.get(dept) {
                Some(vec) => vec,
                None => {
                    println!("Error: No department with name \"{}\" found.", dept);
                    return ();
                }
            };

            let employees = employees.join(", ");
            println!("Employees in {}: {}", dept, employees);
        }

        fn add_employee(dir: &mut Directory, txt: &mut std::str::SplitWhitespace) {
            /*
                Getting new employee name and the department they are going to

                Passing 'txt' not '&mut txt' because 'txt' is already a mutable reference
            */
            let name = match Directory::get_name(txt, Option::Some("to")) {
                Return::Some(s) => s,
                Return::Empty => {
                    println!("Error: Could not find employee name.");
                    return ();
                },
                Return::None => return (),
            };

            let department = match Directory::get_name(txt, Option::None) {
                Return::Some(s) => s,
                Return::Empty => {
                    println!("Error: Could not find department name.");
                    return ();
                }
                Return::None => return (),
            };

            let mut coworkers = dir.employees_by_department.entry(department.clone()).or_insert(Vec::new());
            if coworkers.contains(&name) {
                println!("Error: Employee with that name already exists in that department.");
            } else {
                // Adding employee to list of employees in department
                coworkers.push(name.clone());

                // Adding employee and departmet to list of all employees
                dir.all_employees.push(format!("{} ({})", name, department));
            }
        }

        // Returns an Option<String> instead of Option<&str> because something about Lifetimes
        fn get_name(txt: &mut std::str::SplitWhitespace, stop: Option<&str> ) -> Return<String> {
            let mut name = String::new();
            loop {
                match stop {
                    // If stop at special word
                    Option::Some(s) => {
                        match txt.next() {
                            Some(t) => {
                                // If string is the stopping string, break, else, add string to name
                                if s == t {
                                    break;
                                } else {
                                    name = format!("{}{} ", name, t);
                                }
                            },

                            None => {
                                println!("Error: Expected keyword \"{}\" after employee name.", s);
                                return Return::None;
                            },
                        };
                    },
                    // If stop at end of text
                    Option::None => {
                        match txt.next() {
                            Some(t) => {
                                name = format!("{}{} ", name, t);
                            }, 
                            None => break
                        };
                    }
                };
            }
            if name.len() == 0 {
                return Return::Empty;
            }

            Return::Some(name)
        }
    }
}
