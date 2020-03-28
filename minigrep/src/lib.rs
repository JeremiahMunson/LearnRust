use std::{fs, error::Error};

// Extracting logic from main
// Box<dyn Error> is trait object (covered in Chapter 17)
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // fs::read_to_string returns Result<String>
    // ? covered in Chapter 9 returns error value from current function
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

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
}