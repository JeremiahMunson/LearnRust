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
}
