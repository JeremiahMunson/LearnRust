/*
    This project is a recap of the many skill you've learned so
    far (through Chapter 11) and an exploration of a few more standard
    library features. We'll build a command line tool that interacts
    with file and command line input/output to practice some of the
    Rust concepts you now have under your belt.

    Rust's speed, safety, single binary output, and cross-platform
    support make it an ideal language for creating command line
    tools, so for our project, we'll make our own version of the
    classic command line tool `grep` (Globally search a Rgular
    Expression and Print). In the simplest use case, `grep`, searches
    a specified file for a specified string. To do so, `grep` takes as
    its arguments a filename and a string. Then it reads the 
    file, finds lines in that file that contain the string
    argument, and prints those lines.

    Along the way, we'll show how to make our command line tool use
    features of the terminal that many command line tools use. We'll
    read the value of an environment variable to allow the user to
    configure the behavior of our tool. We'll also print error
    messages to the standard error console stream (stderr) instead
    of standard output (stdout), so, for example, the user can
    redirect successful output to a file while still seeing error
    messages onscreen.

    One Rust community member, Andrew Gallant, has already created a
    fully featured, very fast version of `grep`, called `ripgrep`. By
    comparison, our version of `grep` will be fairly simple, but this
    chapter will give you some of the background knowledge you need to
    understand a real-world project such as `ripgrep`.

    Our `grep` project will combine a number of concepts you've learned
    so far:
        - Organizing code (Chapter 7)
        - Using vectors & strings (Chapter 8)
        - Handling errors (Chapter 9)
        - Using traits and lifetimes where appropriate (Chapter 10)
        - Writing tests (Chapter 11)

    We'll also briefly introduce closures, iterators, and trait
    objects, which Chapters 13 and 17 will cover in detail.
*/

// The function std::env::args enables minigrep to read the values of 
// command line arguments. This function returns an iterator of the 
// command line arguments that were given.
use std::{env, process};
use minigrep::Config;

fn main() {
    // The collect method turns iterator into a collection (like vector)
    let args: Vec<String> = env::args().collect();

    // Calls Config::new() then either retrieves what's in Ok() or
    // if Err() prints the problem and ends the program
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    
    // Only really want to do somehting if an Err(), no other case, so
    // we use if let
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}