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
