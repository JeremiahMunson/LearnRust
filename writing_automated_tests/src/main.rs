fn main() {
    /*
        At its simplest, a test in Rust is a function that's
        annotated with the `test` attribute. Attributes are
        metadata about pieces of Rust code; one example is the
        `derive` attribute we used with structs in Chapter 5. To
        change a function into a test function, add `#[test]` on the
        line before `fn`. When you run your tests with the `cargo test`
        command, Rust builds a test runner binary that runs the
        functions annotated with the `test` attribute and reports on
        whether each test function passes or fails.

        I made a new library that has the same code as making a new
        directory with cargo with `cargo new adder --lib`. With this
        we can run the `cargo test` command and it will run the
        test function(s) in our project.
    */
}
