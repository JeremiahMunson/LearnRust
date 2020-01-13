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
    use std::collections::HashMap;

    pub struct Directory {
        employees_by_department: HashMap<String, Vec<String>>,
        all_employees: Vec<String>,
    }

    // Looked ahead for the general type <T>
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
                    Some("Add") =>      Directory::add_employee(self, &mut split_text),
                    Some("Move") =>     Directory::move_employee(self, &mut split_text),
                    Some("Remove") =>   Directory::remove_employee(self, &mut split_text),
                    Some("Rename") =>   Directory::rename_employee(self, &mut split_text),
                    Some("Print") =>    Directory::print(self, &mut split_text),
                    Some("Help") =>     Directory::help(&mut split_text),
                    Some(s) =>          Directory::check_command(s),
                    None =>             println!("Please enter a command."),
                };
                break;
            }
        }

        // General print function. Checks if user is printing a department and calls print_department(), or prints all the employees
        fn print(&mut self, txt: &mut std::str::SplitWhitespace) {
            match Directory::get_name(txt, Option::None) {
                Return::Some(s) => Directory::print_department(self, &s),
                Return::Empty => {
                    self.all_employees.sort();
                    println!("All employees: {}", self.all_employees.join(", "));
                },
                Return::None => return (),
            };
        }

        // Prints the employees in the selected department
        fn print_department(&mut self, dept: &str) {
            let employees = match self.employees_by_department.get_mut(dept) {
                Some(vec) => vec,
                None => {
                    println!("Error: No department with name \"{}\" found.", dept);
                    return ();
                }
            };
            employees.sort();
            let employees = employees.join(", ");
            println!("Employees in {}: {}", dept, employees);
        }

        // Adds an employee to a department and the entire company
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

            let coworkers = dir.employees_by_department.entry(department.clone()).or_insert(Vec::new());
            if coworkers.contains(&name) {
                println!("Error: Employee with that name already exists in that department.");
            } else {
                // Adding employee to list of employees in department
                coworkers.push(name.clone());

                // Adding employee and departmet to list of all employees
                dir.all_employees.push(format!("{} ({})", name, department));
            }
        }

        // Move an employee from their current department to a new department
        fn move_employee(dir: &mut Directory, txt: &mut std::str::SplitWhitespace) {
            let name = match Directory::get_name(txt, Option::Some("from")) {
                Return::Some(s) => s,
                Return::Empty => {
                    println!("Error: Could not find employee name");
                    return ();
                },
                Return::None => return(),
            };

            let old_department = match Directory::get_name(txt, Option::Some("to")) {
                Return::Some(s) => s,
                Return::Empty => {
                    println!("Error: Could not find input for current department.");
                    return ();
                },
                Return::None => return (),
            };

            let new_department = match Directory::get_name(txt, Option::None) {
                Return::Some(s) => s,
                Return::Empty => {
                    println!("Error: Could not find input for new department.");
                    return ();
                },
                Return::None => return (),
            };


            // Make sure there is no employee with the same name already in the new department
            let new_coworkers = dir.employees_by_department.entry(new_department.clone()).or_insert(Vec::new());
            if new_coworkers.contains(&name) {
                println!("Error: Employee with name \"{}\" already exists in department \"{}\".", name, new_department);
                return ();
            }
            // new_coworkers isn't used again allowing for a new variable with a mutable reference to dir.employees_by_department

            // Make sure there is a department that the employee is supposed to be coming from
            let former_coworkers = match dir.employees_by_department.get_mut(&old_department) {
                Option::Some(v) => v,
                Option::None => {
                    println!("Error: No department \"{}\" exists.", old_department);
                    return ();
                },
            };

            if Directory::is_sorted(&former_coworkers)==false {
                former_coworkers.sort();
            }

            // Finding and removing employee from current department unless the employee doesn't exits in that department
            match former_coworkers.binary_search(&name) {
                Ok(index) => former_coworkers.remove(index),
                Err(_) => {
                    println!("Error: No employee named \"{}\" in department \"{}\".", name, old_department);
                    return ();
                },
            };

            /*
                We have already established that there is no employee with the same name already in the new department and
                that there was an employee with the given name in the old department. Employee already removed from the
                old department.
            */

            // redefining new_coworkers allows for a former_coworkers to be a mutable reference to dir.employees_by_department and
            // for new_coworkers to also be a mutable reference to dir.employees_by_department
            let new_coworkers = match dir.employees_by_department.get_mut(&new_department) {
                Option::Some(v) => v,
                Option::None => return (), // Shouldn't be Option::None because it would have been created above when checking
            };
            new_coworkers.push(name.clone());

            let old_name = format!("{} ({})", name, old_department);
            let new_name = format!("{} ({})", name, new_department);

            Directory::rename_all_employees(dir, old_name, new_name);            
        }

        // Removes an employee from a department and the entire company
        fn remove_employee(dir: &mut Directory, txt: &mut std::str::SplitWhitespace) {
            let name = match Directory::get_name(txt, Option::Some("from")) {
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
                },
                Return::None => return (),
            };

            let employees = match dir.employees_by_department.get_mut(&department) {
                Some(v) => v,
                None => {
                    println!("Error: No such department exists.");
                    return ();
                }
            };

            // Make sure employees is sorted so we can do a binary search
            if Directory::is_sorted(&employees)==false {
                employees.sort();
            }
            match employees.binary_search(&name) {
                Ok(index) => {
                    employees.remove(index);
                    if employees.len() == 0 {
                        dir.employees_by_department.remove(&department);
                    }
                },
                Err(_) => {
                    println!("Error: No employee by that name exists in that department.");
                    return ();
                },
            };



            let employee = format!("{} ({})", name, department);

            // Make sure all employees is sorted so we can do a binary search
            if Directory::is_sorted(&dir.all_employees)==false {
                dir.all_employees.sort();
            }
            match dir.all_employees.binary_search(&employee) {
                Ok(index) => dir.all_employees.remove(index),
                Err(_) => {
                    println!("Error: Could not find employee in all employees.");
                    return();
                },
            };
        }

        // Rename an employee
        fn rename_employee(dir: &mut Directory, txt: &mut std::str::SplitWhitespace) {
            let old_name = match Directory::get_name(txt, Option::Some("in")) {
                Return::Some(s) => s,
                Return::Empty => {
                    println!("Error: Could not find input for employee name.");
                    return ();
                },
                Return::None => return (),
            };

            let department = match Directory::get_name(txt, Option::Some("to")) {
                Return::Some(s) => s,
                Return::Empty => {
                    println!("Error: Could not find input for department name.");
                    return ();
                },
                Return::None => return (),
            };

            let new_name = match Directory::get_name(txt, Option::None) {
                Return::Some(s) => s,
                Return::Empty => {
                    println!("Error: Could not find input for new name.");
                    return ();
                },
                Return::None => return (),
            };

            let coworkers = match dir.employees_by_department.get_mut(&department) {
                Option::Some(v) => v,
                Option::None => {
                    println!("Error: No department named \"{}\" found.", department);
                    return ();
                },
            };

            if Directory::is_sorted(&coworkers)==false {
                coworkers.sort();
            }
            match coworkers.binary_search(&old_name) {
                Ok(index) => coworkers[index] = new_name.clone(),
                Err(_) => {
                    println!("Error: No employee named \"{}\" in department \"{}\".", old_name, department);
                    return ();
                },
            };

            let old_name = format!("{} ({})", old_name, department);
            let new_name = format!("{} ({})", new_name, department);

            Directory::rename_all_employees(dir, old_name, new_name);
        }

        fn help(txt: &mut std::str::SplitWhitespace) {
            match txt.next() {
                Option::Some(_) => println!("Error: No text should follow \"Help\"."),
                Option::None => {
                    println!("Add Employee:\t\t\t\"Add 'name' to 'department'\"");
                    println!("Move Employee:\t\t\t\"Move 'name from 'old department' to 'new department'\"");
                    println!("Remove Employee:\t\t\"Remove 'name' from 'department'\"");
                    println!("Print Employees in Department:\t\"Print 'department'\"");
                    println!("Print All Employees:\t\t\"Print\"");
                    println!("You don't need to add single quotes around names and departments. They can also be more than one word.");
                },
            };
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

            // removes the space at the end of the name from formatting
            name.pop();

            Return::Some(name)
        }

        // Checks if a vector of Strings is sorted or not
        fn is_sorted(vector: &Vec<String>) -> bool {
            let mut sorted = true;
            let mut iterator = vector.iter();
            match iterator.next() {
                None => {
                    println!("Error: Empty Vector");
                    return false;
                },
                Some(s) => {
                    let mut previous = s;
                    for i in iterator {
                        if i < previous {
                            sorted = false;
                            break;
                        }
                        previous = i;
                    }
                }
            }

            sorted
        }

        // Checks if input command is similar to an existing command
        fn check_command(txt: &str) {
            if txt=="add" || txt=="ADD" {
                println!("Error: No command \"{}\" found. Did you mean \"Add\"?", txt);
            } else if txt=="remove" || txt=="REMOVE" {
                println!("Error: No command \"{}\" found. Did you mean \"Remove\"?", txt);
            } else if txt=="print" || txt=="PRINT" {
                println!("Error: No command \"{}\" found. Did you mean \"Print\"?", txt);
            } else {
                println!("Error: No command \"{}\" found. Try \"Help\" for a list of commands.", txt);
            }
        }

        fn rename_all_employees(dir: &mut Directory, old_name: String, new_name: String) {
            if Directory::is_sorted(&dir.all_employees)==false {
                dir.all_employees.sort();
            }

            match dir.all_employees.binary_search(&old_name) {
                Ok(index) => dir.all_employees[index] = new_name, // new_name now no longer in scope
                Err(_) => println!("Error: Was unable to rename employee in list of all employees."),
            };
        }
    }
}
