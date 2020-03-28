use std::{fs, error::Error};

// Extracting logic from main
// Box<dyn Error> is trait object (covered in Chapter 17)
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // fs::read_to_string returns Result<String>
    // ? covered in Chapter 9 returns error value from current function
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config{
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // Index 0 is the program's name, so we start with index 1 then
        // index 2 for the input
        let query = args[1].clone();
        let filename = args[2].clone();
        // Many Rustaceans avoid using `clone` because of its runtime cost.
        // We'll learn more efficient methods in Chapter 13

        Ok(Config { query, filename })
    }
}

// Because search returns string slices of contents, we need to make 
// sure the lifetime of the return value matches the lifetime of the
// contents
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    // lines is a method that allows iteration over lines of a string
    for line in contents.lines() {
        // contains checks if string contains the input
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

/*
    Test-driven development (TDD) process:
        1. Write a test that fails and run it to make sure it fails for the reason you expect.
        2. Write or modiy just enough code to make the new test pass.
        3. Refactor the code you just added or changed and make sure the tests continue to pass.
        4. Repeat from step 1.
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_config_pass() {
        let arr = ["zero".to_string(), "one".to_string(), "two".to_string()];
        let config = Config::new(&arr).unwrap_or_else(|err| {
            panic!("Error: {}", err);
        });
        assert_eq!(arr[1], config.query);
        assert_eq!(arr[2], config.filename);
    }

    #[test]
    #[should_panic(expected = "Error: not enough arguments")]
    fn new_config_too_short() {
        let arr = ["zero".to_string(), "one".to_string()];
        let config = Config::new(&arr).unwrap_or_else(|err| {
            panic!("Error: {}", err);
        });
        assert_eq!(arr[0], config.query);
        assert_eq!(arr[1], config.filename);
    }

    #[test]
    fn run_pass() {
        let arr = ["zero".to_string(), "the".to_string(), "poem.txt".to_string()];
        let config = Config::new(&arr).unwrap_or_else(|err| {
            panic!("Error: {}", err);
        });

        if let Err(e) = run(config) {
            panic!("Application error: {}", e);
        }
    }

    #[test]
    #[should_panic(expected = "Application error")]
    fn run_no_file() {
        let arr = ["zero".to_string(), "the".to_string(), "test.txt".to_string()];
        let config = Config::new(&arr).unwrap_or_else(|err| {
            panic!("Error: {}", err);
        });

        if let Err(e) = run(config) {
            panic!("Application error: {}", e);
        }
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        // This test searches for the string "duct" in contents.
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}