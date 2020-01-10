// Bring Rust's HashMap into scope
use std::collections::HashMap;

use rand::Rng;

/*
    If we're using multiple items defined in the same package or
    same module, listing each item can take up a lot of vertical
    space in our files. Instead, we can use nested paths to bring
    the same items into scope in one line. We do this by specifying
    the common part of the path, followed by two colons, and then
    curly brackets around a list of the parts of the paths that
    differ.
*/
// use std::io;
// use std::cmp::Ordering;
// becomes
// use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;
// becomes
use std::io::{self, Write};

// If we want to bring _all_ public items defined in a path into
// scope, we can specify that path followd by '*', the glob operator.
use std::collections::*;
// Be careful using the glob operator! Glob can make it harder to tell
// what names are in scope and where a name used in your program was
// defined. The glob operator is often used when testing to bring
// everything under test into the 'tests' module.


fn main() {
    let mut map = HashMap::new();
    map.insert(1,2);


    /*
        We can use external packages too. Like from the project in
        chapter 2. We go to the Cargo.toml and add rand into the 
        dependencies to tell Cargo to download the 'rand' package and 
        any dependenices from crates.io and make rand available to our 
        project. We then bring rand definitions into the scope of our 
        package by adding 'use' line.

        Members of the Rust community have made many packages available at
        crates.io, and pulling any of them into your package involves these
        same steps: listing them in your package's Cargo.toml file and using
        'use' to bring items into scope.

        Note that the standard library ('std') is also a crate external to 
        our package, but the standard library is shipped with the Rust
        language so we don't need to change Cargo.toml to include std. But
        we do still need to use 'use' to bring it into scope
    */
    let secret_number = rand::thread_rng().gen_range(1,101);
}

/*
    We can bring two types of the same name into
    the same scope with use: after the path, we
    specify 'as' and a new local name, or alias.
*/

// This code doesn't work. It calls an error because
// nothing is actually being returned.
/*
use std::fmt::Result;
use std::io::Result as IoResult;
fn function1() -> Result {
    // --snip--
}
fn function2() -> IoResult<()> {
    // --snip--
}
*/