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

    pub enum Next {
        CONT,
        BREAK,
    }

    pub fn update_employees(employees_by_department: &mut HashMap<String, Vec<String>>) -> Next {
        let mut user_input;

        loop { 
            user_input = String::new();
            io::stdin().read_line(&mut user_input).expect("Failed to read line.");
            let split_text = user_input.split_whitespace();
            let mut words: LinkedList<&str> = LinkedList::new();
            for word in split_text {
                words.push_back(word);
            }
            match words.pop_front() {
                Some("Add") => {
                    match add_employee(&mut words, employees_by_department) {
                        Next::CONT => {println!("Error: Could not add employee. Try again."); continue},
                        Next::BREAK => break,
                    }
                },
                Some("Remove") => {
                    match remove_employee(&mut words, employees_by_department) {
                        Next::CONT => {println!("Error: Could not remove employee. Try again."); continue},
                        Next::BREAK => break
                    }
                },
                Some("Move") => {
                    match move_employee(&mut words, employees_by_department) {
                        Next::CONT => {println!("Error: Could not move employees. Try again."); continue},
                        Next::BREAK => break
                    }
                },
                Some("Rename") => {println!("Renaming"); break},
                Some("Print") => {
                    match print(&mut words, employees_by_department) {
                        Next::CONT => {println!("Error: Could not print employees. Try again."); continue},
                        Next::BREAK => break
                    }
                },
                Some("Help") => {
                    help_info(); 
                    break;
                },
                Some("Exit") => {
                    return Next::BREAK;
                },
                _ => continue,
            };
        }

        Next::CONT
    }

    fn help_info() {
        println!("Add employee: 'Add name to dept'");
        println!("Remove employee: 'Remove name from dept'");
        println!("Move employee: 'Move name from old_dept to new_dept'")
        println!("Rename employee: 'Rename old_name in dept to new_name")
        println!("Print employees from department: 'Print dept'");
        println!("Print all employees: 'Print'");
        println!("Leave program: 'Exit'");

        println!("Names and departments can be multiple words.");

    }

    fn add_employee(txt: &mut LinkedList<&str>, employees: &mut HashMap<String, Vec<String>>) -> Next{
        let mut name = String::new();
        // Loop through the input until we reach "to". Everything before that is a name
        loop {
            let next = txt.pop_front();
            // Check if next word is "to", something else, or doesn't exist
            match next {
                // If the next word is "to", leave the loop
                Some("to") => break,
                // If the next word exists, add it to the name
                Some(i) => {
                    name = format!("{}{} ", name, i)
                },
                // If there is no next word, return Next::CONT because it can't add employee
                None => return Next::CONT,
            };
        }

        // If the length of name is 0 then there was no name input
        if name.len() == 0 {
            return Next::CONT;
        }
        name.pop(); // Removes space added from formating

        let mut department_name = String::new();
        // Loop through the input until we reach the end of the string. Everything is
        // the department name
        loop {
            let next = txt.pop_front();
            match next {
                Some(i) => {
                    department_name = format!("{}{} ", department_name, i)
                }
                None => break
            };
        }

        // If the length of department_name is 0 then there was no department name input
        if department_name.len() == 0 {
            return Next::CONT;
        }

        department_name.pop(); // Removes space added from formatting

        let name_cloned = name.clone();
        let department_cloned = department_name.clone();
        
        
        let emps = employees.entry(department_name).or_insert(Vec::new());
        // department_name no longer in scope


        // .contains() would expect &&name but &&name is type &&String and it wants &&str
        // I got around this by making a reference to name as a variable because 
        // Rust can force &String to act like &str but apparently not &&String to &&str
        if emps.contains(&name) {
            println!("An employee with that name already exists in this department");
            return Next::CONT;
        }

        // If the vector already has the name it would have returned, leaving the function and
        // not reaching this point
        emps.push(name);
        // name no longer in scope
        emps.sort();

        println!("Added {} to {}", name_cloned, department_cloned);

        Next::BREAK
    }
    
    fn remove_employee(txt: &mut LinkedList<&str>, employees: &mut HashMap<String, Vec<String>>) -> Next {
        let mut name = String::new();
        // Loop through the input until we reach "to". Everything before that is a name
        loop {
            let next = txt.pop_front();
            // Check if next word is "to", something else, or doesn't exist
            match next {
                // If the next word is "to", leave the loop
                Some("from") => break,
                // If the next word exists, add it to the name
                Some(i) => {
                    name = format!("{}{} ", name, i)
                },
                // If there is no next word, return Next::CONT because it can't add employee
                None => return Next::CONT,
            };
        }

        // If the length of name is 0 then there was no name input
        if name.len() == 0 {
            return Next::CONT;
        }
        name.pop(); // Removes space added from formating

        let mut department_name = String::new();
        // Loop through the input until we reach the end of the string. Everything is
        // the department name
        loop {
            let next = txt.pop_front();
            match next {
                Some(i) => {
                    department_name = format!("{}{} ", department_name, i)
                }
                None => break
            };
        }

        // If the length of department_name is 0 then there was no department name input
        if department_name.len() == 0 {
            return Next::CONT;
        }

        department_name.pop(); // Removes space added from formatting

        let emps = match employees.get_mut(&department_name) {
            Some(v) => v,
            None => return Next::CONT,
        };

        let location_employee = emps.binary_search(&name);
        match location_employee {
            Ok(loc) => emps.remove(loc),
            _ => return Next::CONT,
        };
        
        if emps.len() == 0 {
            employees.remove(&department_name);
        }

        println!("Removed {} from {}", name, department_name);

        Next::BREAK
    }

    fn print(txt: &mut LinkedList<&str>, employees: &mut HashMap<String, Vec<String>>) -> Next {
        let mut dep = String::new();
        loop {
            let next = txt.pop_front();
            let new_word = match next {
                Some(i) => i,
                None => break,
            };
            dep = format!("{}{} ", dep, new_word);
        }

        if dep.len() == 0 {
            return print_everyone(employees);
        }

        // Remove space from formatting
        dep.pop();

        // Get vector of people from the hash map
        let dep_employees = match employees.get(&dep) {
            Some(txt) => txt,
            None => return Next::CONT,
        };

        let mut people = String::new();
        for person in dep_employees {
            people = format!("{}{}, ", people, person);
        }
        people.pop();
        people.pop();

        println!("Employees in {}: {:?}", dep, people);

        Next::BREAK
    }

    fn print_everyone(employees: &mut HashMap<String, Vec<String>>) -> Next{
        let mut everyone:  Vec<String> = Vec::new();
        let mut all_employees = employees.clone();
        for val in all_employees.values_mut() {
            everyone.append(val);
        }

        everyone.sort();
        let mut people = String::new();
        for person in everyone {
            people = format!("{}{}, ", people, person);
        }

        people.pop();
        people.pop();

        println!("All Employees: {}", people);

        Next::BREAK
    }

    fn move_employee(txt: &mut LinkedList<&str>, employees: &mut HashMap<String, Vec<String>>) -> Next {
        let mut name = String::new();
        // Loop through the input until we reach "in". Everything before that is a name
        loop {
            let next = txt.pop_front();
            // Check if next word is "in", something else, or doesn't exist
            match next {
                // If the next word is "in", leave the loop
                Some("from") => break,
                // If the next word exists, add it to the name
                Some(i) => {
                    name = format!("{}{} ", name, i)
                },
                // If there is no next word, return Next::CONT because it can't add employee
                None => return Next::CONT,
            };
        }

        // If the length of name is 0 then there was no name input
        if name.len() == 0 {
            return Next::CONT;
        }
        name.pop(); // Removes space added from formating

        //println!("Name: {}", name);

        let mut old_department = String::new();
        // Loop through the input until we reach "to". Everything is
        // the current department name
        loop {
            let next = txt.pop_front();
            match next {
                Some("to") => break,
                Some(i) => {
                    old_department = format!("{}{} ", old_department, i)
                },
                None => return Next::CONT,
            };
        }

        // If the length of old_department is 0 then there was no department name input
        if old_department.len() == 0 {
            return Next::CONT;
        }

        old_department.pop(); // Removes space added from formatting


        let mut new_department = String::new();
        // Loop through the input until we reach the end of the string. Everything is
        // the new department name
        loop {
            let next = txt.pop_front();
            match next {
                Some(i) => {
                    new_department = format!("{}{} ", new_department, i);
                },
                None => break,
            }
        }

        // If the length of new_department is 0 then there was no department name input
        if new_department.len() == 0 {
            return Next::CONT;
        }

        new_department.pop();

        //let name_cloned = name.clone();
        //let old_cloned = old_department.clone();
        //let new_cloned = new_department.clone();


        let emps = match employees.get_mut(&old_department) {
            Some(v) => v,
            None => return Next::CONT,
        };

        let location_employee = emps.binary_search(&name);
        match location_employee {
            Ok(loc) => emps.remove(loc),
            _ => return Next::CONT,
        };

        let new_emps = employees.entry(new_department.clone()).or_insert(Vec::new());
        new_emps.push(name.clone());
        new_emps.sort();


        println!("Moved {} from {} to {}", name, old_department, new_department);


        Next::BREAK
    }
}
