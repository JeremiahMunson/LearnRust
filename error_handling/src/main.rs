/*
    Rust groups errors into two major categories: recoverable and
    unrecoverable errors. For a recoverable error, such as a
    file not found error, it's reasonable to report the problem to
    the user and retry the operation. Unrecoverable errors are always
    symptoms of bugs, like trying to access a location beyond the
    end of an array.

    Most languages don't distinguish between these two kinds of
    errors and handle both in the same way, using mechanisms such as
    exceptions. Rust doesn't have exceptions. Instead, it has the
    type 'Result<T, E>' for recoverable errors and the 'panic!'
    macro that stops execution when the program encounters an
    unrecoverable error. This chapter covers calling 'panic!' first
    and then talks about returning 'Result<T, E>' values. Additionally,
    we'll explore considerations when deciding whether to try to
    recover from an error or to stop execution.
*/
use std::fs::File;
use std::io::{self, ErrorKind, Read};


fn main() {
    /*
        When bad things happen in your code and there's nothing you can
        do about it, Rust has the 'panic!' macro. When the 'panic!'
        macro executes, your program will print a failure message, unwind
        and clean up the stack, and then quit. This most commonly occurs
        when a bug of some kind has been detected and it's not clear to
        the programmer how to handle the error.

        By default, when a panic occurs, the program starts 
        unwinding, which means Rust walks back up the stack and cleans
        up the data from each function it encounters. But this walking
        back and cleanup is a lot of work. The alternative is to
        immediately abort, which ends the program without cleaning
        up. Memory that the program was using will then need to be
        cleaned up by the operating system. If in your project you need
        to make the resulting binary as small as possible, you can
        switch from unwinding to aborting upon a panic by adding
        panic = 'abort'
        to the appropriate '[profile]' sections in our Cargo.toml
        file. For example, if you want to abort on panic in release
        mode, add this:

        [profile.release]
        panic = 'abort'
    */

    // Let's try calling 'panic!' in a simple program
    //panic!("crash and burn");

    /*
        The call to 'panic!' causes the error message contained in the
        last two lines. The first line shows our panic message and the
        place in our source code where the panic occured:
        src/main.rs:48:5
        indicates that it's the 48th line, 5th character of sr/main.rs

        In this case, the line indicated is part of our code, and if
        we go to that line, we see the 'panic!' macro call. In other
        cases, the 'panic!' call might be in code that our code
        calls, and the filename and line number reported by the error
        message will be someone else's code where the 'panic!' macro is
        called, not the line of our code that eventually led to the
        'panic!' call. We can use the backtrace of the functions the
        'panic!' call came from to figure out the part of our code that
        is causing the problem.

        Let's look at another example to see what it's like when a
        'panic!' call comes from a library because of a bug in our
        code instead of from our code calling the macro directly. The
        code below has some code that attempts to access an element by
        index in a vector.
    */

    /*
        Here, we're attempting to access the 100th element of our
        vector, but it has only 3 elements. In this situation, Rust
        will panic. Using '[]' is supposed to return an element, but if
        you pass an invalid index, there's no element that Rust could
        return here that would be correct.

        Other languages, like C, will attempt to give you exactly what
        you asked for in this situation, even though it isn't what you
        want: you'll get whatever is at the location in memory that would
        correspond to that element in the vector, even though the
        memory doesn't belong to the vector. This is called a buffer
        overread and can lead to security vulnerabilities if an attacker
        is able to manipulate the index in such a way as to read data
        they shouldn't be allowed to that is stored after the array.

        To protect your program from this sort of vulnerability, if you
        try to read an element at an index that doesn't exist, Rust will
        stop execution and refuse to continue.
    */
    //let v = vec![1, 2, 3];
    //v[99];
    /*
        This error points at a file we didn't write, libcore/slice/mod.rs.
        That's the implementation of 'slice' in the Rust source code. The
        code that gets run when we use '[]' on our vector 'v' is in
        libcore/slice/mod.rs, and that is where the 'panic!' is actually
        happening.

        The next note line tells us that we can set the 'RUST_BACKTRACE'
        environment variable to get a backtrace of exactly what
        happened to cause the error. A _backtrace_ is a list of all the
        functions that have been called to get to this point. Backtraces
        in Rust work as they do in other languages: the key to reading
        the backtrace is to start from the top and read until you see
        files you wrote. That's the spot where the problem
        originated. The lines above the lines mentioning your files are
        code that your code called; the lines below are code that
        called your code. These lines might include core Rust
        code, standard library code, or crates that you're using. Let's
        try getting a backtrace by setting the 'RUST_BACKTRACE'
        environment variable to any value except 0.

        Run: RUST_BACKTRACE=1 cargo run
    */

    /*
        Most errors aren't serious enough to require the program to
        stop entirely. Sometimes, when a function fails, it's for a
        reason that you can easily interpret and respond to. For
        example, if you try to open a file and that operation
        fails because the file doesn't exist, you might want to
        create the file instead of terminating the process.

        The 'Result' enum is defined as having two variants, 'Ok' and
        'Err' as follows:

        enum Result<T, E> {
            Ok(T),
            Err(E),
        }

        The 'T' and 'E' are generic types parameters. What you need to
        know right now is that 'T' represents the type of the value
        that will be returned in a success case within the 'Ok'
        variant, and 'E' represents the type of the error that will be
        returned in a failure case within the 'Err' variant. Because
        'Result' has these generic type parameters, we can use the
        'Result' type and the functions that the standard library has
        defined on it in many different situations where the successful
        value and error value we want to return may differ.

        Let's call a function that returns a 'Result' value because the
        function could fail. In the code below we try to open a file.
    */

    //let f = File::open("hello.txt");

    /*
        How do we know 'File::open' returns a 'Result'? We could look
        at the standard library API documentation, or we could ask the
        compiler! If we give f a type annotation that we know is _not_
        the return type of the function and then try to compile the
        code, the compiler will tell us that the types don't match. The
        error message will then tell us what the type of 'f' _is_. Let's
        try it! We know that the return type of 'File::open' isn't of
        type 'u32', so let's change the 'let f' statement to 

        let f: u32 = File::open("hello.txt");

        Attempting to compile now gives us the following (abbreviated)
        output:

        ^^^^^^^^^^^ expected u32, found enum 'std::result::Result`

        = note: expected type 'u32'
                   found type 'std::result::Result<std::fs::File, std::io::Error>'

        

        This tells us the return type of the `File::open` function is a
        `Result<T, E>`. The generic parameter `T` has been filled
        in here with the type of the success value, `std::fs::File`, which
        is a file handle. The type of `E` used in the error value is
        `std::io::Error`.

        This return type means the call to `File::open` might succeed and
        return a file handle that we can read from or write to. The
        function call also might fail: for example, the file might not
        exist, or we might not have permission to access the file. The
        `File::open` function needs to have a way to tell us 
        information. This information is exactly what the `Result` enum
        conveys.

        In the case where `File::open` succeeds, the value in the
        variable `f` will be an instance of `Ok` that contains a file
        handle. In the case where it fails, the value in `f` will be an
        instance of `Err` that contains more information about the kind
        of error that happened.

        We need to add to the code above to take different
        actions depending on the value `File::open` returns. The code
        below shows one way to handle the `Result` using a basic
        tool, the `match` expression.
    */

    //let f = match f {
    //    Ok(file) => file,
    //    Err(error) => {
    //        panic!("Problem opening the file: {:?}", error);
    //    },
    //};

    /*
        Note that, like the `Option` enum, the `Result` enum and its
        variants have been brought into scope by the prelude, so we don't
        need to specify `Result::` before the `Ok` and `Err` variants in
        the `match` arms.

        Here we tell Rust that when the result is `Ok`, return the inner
        `file` value out of the `Ok` variant, and we then assign that
        file handle value to the variable `f`. After the `match`, we can
        use the file handle for reading or writing.

        The other arm of the `match` handles the case where we get an
        `Err` value from `File::open`. In this example, we've chosen to
        call the `panic!` macro. If there's no file named hello.txt in
        our current directory and we run this code, we'll see the
        following output from the `panic!` macro: (abbrev.)

        thread 'main' panicked at 'Problem opening the file: Error { '
    */

    /*
        The code above will `panic!` no matter why `File::open`
        failed. What we want to do instead is take different actions for
        different failure reasons: if `File::open` failed because the
        file doesn't exist, we want to create the file and return the
        handle to the new file. If `File::open` failed for any other
        reason - for example, because we didn't have permission to open
        the file - we still want the code to `panic!` in the same way as
        it did above.
    */

    /*
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file {:?}", other_error),
        },
    };
    */

    /*
        That’s a lot of match! The match expression is very useful but 
        also very much a primitive. In Chapter 13, you’ll learn about 
        closures; the Result<T, E> type has many methods that accept a 
        closure and are implemented using match expressions. Using those
        methods will make your code more concise. A more seasoned 
        Rustacean might write this code instead:
    */
    /*
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
    */
    /*
        We can use the method `.unwrap()` as a shorthad for the match
        expressions used that returns the data if `Ok` and calls panic for
        us if it is and `Err`
    */
    //let f = File::open("hello.txt").unwrap();
    /*
        Another method, `.expect()`, which is similar to unwrap, lets us
        choose the `panic!` error message. Using `expect` instead of
        `unwrap` and providing good error messages can convey your 
        intent and make tracking down the source of a panic easier. The
        syntax of `expect` looks like this:
    */
    //let f = File::open("hello.txt").expect("Failed to open hello.txt");

    /*
        When you’re writing a function whose implementation calls 
        something that might fail, instead of handling the error within 
        this function, you can return the error to the calling code so 
        that it can decide what to do. This is known as propagating the
        error and gives more control to the calling code, where there 
        might be more information or logic that dictates how the 
        error should be handled than what you have available in the 
        context of your code. For example, the function
        fn read_username_from_file()

        The pattern of propagating errors is so common in Rust that
        Rust provides the question mark operator `?` to make this
        easier. The `?` placed after a `Result` value is defined to
        work in almost the same way as the `match` expressions we
        defined to handle the `Result` values from before. If the value
        of the `Result` is an `Ok`, the value inside the `Ok` will get
        returned from this expression, and the program will continue. If
        the value is an `Err`, the `Err` will be returned from the whole
        function as if we had used the return keyword so the error value
        gets propagated to the calling code.

        We are only allowed to use the `?` operator in a function that
        returns `Result` or `Option` or another type that implements
        `std::ops::Try`. This means you can't use it in main
        normally. However, main can have a return type of (), and
        conveniently, another valid return type is `Result<T, E>` as
        shown below.

        fn main() -> Result<(), Box<dyn Error>> {
            let f = File::open("hello.txt")?;

            Ok(())
        }

        The `Box<dyn Error>` type is called a trait object. For 
        now you can read `Box<dyn Error>` to mean "any kind of
        error". Using `?` in a main function with this return type is
        allowed.
    */
}

/*
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
*/

fn read_username_from_file() -> Result<String, io::Error> {
    //let mut f = File::open("hello.txt")?;
    //let mut s = String::new();
    //f.read_to_string(&mut s)?;
    // Instead of creating variable f you can chain it like below
    //File::open("hello.txt")?.read_to_string(&mut s)?;
    //Ok(s)

    // The above is so common that Rust has created the following
    // function that creates a new string, reads the contents of the
    // file, puts the contents into that string, and returns it.

    fs::read_to_string("hello.txt")
}