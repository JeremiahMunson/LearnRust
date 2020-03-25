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

    /*
        Just as `cargo run` compiles your code and then runs the
        resulting binary, `cargo test` compiles your code in test 
        mode and runs the resulting test binary. You can specify
        command line options to change the default behavior of
        `cargo test`. For example, the default behavior of the
        binary produced by `cargo test` is to run all the tests in
        parallel and capture output generated during test
        runs, preventing the output from being displayed and making it
        easier to read the output related to the test results.

        Some command line options go to `cargo test`, and some go to the
        resulting test binary. To separate these two types of
        arguments, you list the arguments that go to `cargo test` followed
        by the separator `--` and then the ones that go to the test
        binary. Running `cargo test --help` displays the options you can
        use with `cargo test`, and running `cargo test -- --help` displays
        the options you can use after the separator `--`.

        When you run multiple tests, by default they run in parallel using
        threads. This means the tests will finish running faster so you
        can get feedback quicker on whether or not your code is 
        working. Because the tests are running at the same time, make
        sure your tests don't depend on each other or on any shared
        state, including a shared environment, such as the current
        working directory or environment variables.

        For example, say each of your tests run some code that creates
        a file on disk named test-output.txt and writes some data to
        that file. Then each test reads the data in that file and
        asserts that the file contains a particular value, which is
        different in each test. Because the tests run at the same
        time, one test might overwrite the file between when another
        test writes and reads the file. The scond test will then 
        fail, not because the code is incorrect but because the tests
        have interfered with each other while running in parallel. One
        solution is to make sure each test writes to a different
        file; another solution is to run the tests one at a time.

        If you don't want to run the tests in parallel or if you want
        more fine-grained control over the number of threads used, you
        can send the `--test-threads` flag and the number of threads you
        want to use to the test binary.
        `cargo test -- --test-threads=1`
        We set the number of test threads to `1`, telling the program
        not to use any parallelism. Running the tests using one thread
        will take longer than running them in parallel, but the tests
        won't interfere with each other if they share state.

        When testing, no printing happens on passed functions, only
        tests that fail print whatever was printed in the function. To
        see printed values for passing tests we can disable the 
        output capture behaviour by using the `--nocapture` flag:
        `cargo test -- --nocapture`

        Sometimes, running a full test suite can take a long time. If
        you're working on code in a particular area, you might want to
        run only the tests pertaining to that code. You can choose which
        tests to run by passing `cargo test` the name or names of the
        test(s) you want to run as an argument. To demonstrate how to
        run a subset of tests, we'll create three tests for
        our `add_two` function, and choose which ones to run.
        (test: add_two_and_two, add_three_and_two, one_hundred)
        We can pass the name of any test function to `cargo test` to
        run only that test.
        `cargo test one_hundred`
        Runs just the test with function name `one_hundred`). The output
        also alerts us to how many tests were filtered out.

        We can specify part of a test name, and any test whose name
        matches that value will be run. For example, because several of
        our tests' names contain `add`, we can run those by running
        `cargo test add`
        This command ran all tests with `add` in the name and filtered
        out the remaining tests. Also note that the module in which a
        test appears becomes part of the test's name, so we can run
        all the tests in a module by filtering on the module's name.

        Sometimes a few specific tests can be very time-consuming to
        execute, so you might want to exclude them during most runs
        of `cargo test`. Rather than listing as arguments all tests
        you do want to run, you can instead annotate the
        time-consuming tests using the `ignore` attribute to exclude
        them, as shown with `expensive_test`. After #[test] we add
        the #[ignore] line to the test we want to exclude. Now when
        we run our tests `expensive_test` doesn't run.

        The `expensive_test` function is listed as `ignored`. If we
        want to run only the ignored tests, we can use
        `cargo test -- --ignore`
        By controlling which tests run, you can make sure your
        `cargo test` results will be fast. When you're at a point where
        it makes sense to check the results of the `ignored` tests and
        you have time to wait for the results, you can run 
        `cargo test -- --ignore` instead.

        TEST ORGANIZATION

        As mentioned at the start of the chapter, testing is a
        complex discipline, and different people use different
        terminology and organization. The Rust community thinks about
        tests in terms of two main categories: unit tests and
        integration tests. Unit tests are small and more focused, testing
        one module in isolation at a time, and can test private
        interfaces. Integration tests are entirely external to your
        library and use your code in the same way any other external
        code would, using only the public interface and potentially
        exercising multiple modules per test. Writing both kinds of
        tests is important to ensure that the pieces of your library are
        doing what you expect them to, separately and together.

        Unit Test:
        The purpose of unit tests is to tesst each unit of code in
        isolation from the rest of the code to quickly pinpoint where
        code is and isn't working as expected. You'll put unit tests in
        the `src` directory in each file with the code that they're
        testing. The convention is to create a module named `tests` in
        each file to contain the test functions and to annotate the
        module with `cfg(test)`.

        The `#[cfg(test)]` annotation on the tests module tells Rust to
        compile and run the test code only when you run `cargo test`, not
        when you run `cargo build`. This saves compile time when you
        only want to build the library and saves space in the resulting
        compiled artifact because the tests are not included. You'll
        see that because integration tests go in a different
        directory, they don't need the `#[cfg(test)]` annotation. However,
        because unit tests go in the same files as the code, you'll use
        `#[cfg(test)]` to specify that they shouldn't be included in
        the compiled result.

        Recall that when we generated the new `adder` project in the
        first section of this chapter, Cargo generated the below code for
        us:
        `
        #[cfg(test)]
        mod tests {
            #[test]
            fn it_works() {
                assert_eq!(2 + 2, 4);
            }
        }
        `
        This code is the automatically generated test module. The
        attribute `cfg` stands for _configuration_ and tells Rust that
        the following item should only be included given a certain 
        configuration option. In this case, the configuration option
        is `test`, which is provided by Rust for compiling and running
        tests. By using the `cfg` attribute, Cargo compiles our test
        code only if we actively run the tests with `cargo test`. This
        includes any helper functions that might be within this
        module, in addition to the functions annotated with `#[test]`.

        Testing Private Functions:
        There's debate within the testing community about whether or not
        private functions should be tested directly, and other languages
        make it difficult or impossible to test private functions. Regardless
        of which testing ideology you adhere to, Rust's privacy rules do
        allow you to test private functions. Consider the private
        function `internal_adder`. The function is not marked as
        `pub`, but because tests are just Rust code and the `tests`
        module is just another module, you can bring `internal_adder`
        into a test's scope and call it. If you don't think private
        functions should be tested, there's nothing in Rust that will
        compel you to do so.

        Integration Tests:
        In Rust, integration tests are entirely external to you
        library. They use your library in the same way any other code
        would, which means they can only call functions that are part of
        your library's public API. Their purpose is to test whether many
        parts of your library work together correctly. Units of code
        that work correctly on their own could have problems when
        integrated, so test coverage of the integrated code is important
        as well. To create integration tests, you first need a _tests_
        directory.

        The _tests_ Directory
        We create a _tests_ directory at the top level of our project
        directory, next to _src_. Cargo knows to look for integration
        test files in this directory. We can then make as many test
        files as we want to in this directory, and Cargo will compile
        each of the files as an individual crate.

        Let's create an individual test. Create a new file named
        `tests/integration_test.rs`. We've added `use adder` at the top
        of the code, which we didn't need in the unit tests. The reason
        is that each file in the `tests` directory is a separate 
        crate, so we need to bring our library into each test crate's
        scope. We don't need to annotate any code in 
        `tests/integration_test.rs` with `#[cfg(test)]`. Cargo treats
        the `tests` directory specially and complies files in this
        directory only when we run `cargo test`. Run `cargo test`. I had
        to ignore all the tests that would fail because it didn't get to
        the integration_test file when the lib.rs file tests failed.

        We can still run particular integration test function by
        specifying the test function's name as an argument to
        `cargo test`. To run all the tests in a particular integration
        test file, use the `--test` argument of `cargo test` followed by
        the name of the file:
        `cargo test --test integration_test`
        This command runs only the tests in the 
        `tests/integration_test.rs` file

        Submodules in Integration Test
        As you add more integration tests, you might want to make more
        than one file in the `tests` directory to help organize them;
        for example, you can group the test functions by the
        functionality they're testing. As mentioned earlier, each
        file in the _test_ directory is compiled as its own separate
        crate.

        Treating each integration test file as its own crate is useful
        to create separate scopes that are more like the way end users
        will be using your crate. However, this means files in the 
        _tests_ directory don't share the same behavior as files in
        _src_ do, as you learned in Chapter 7 regarding how to separate
        code into modules and files.

        The different behavior of files in the _tests_ directory is most
        noticeable when you have a set of helper functions that would be
        useful in multiple integration test files and you try to follow
        the steps in the "Separating Modules into Different Files" 
        section of Chapter 7 to extract them into a common module. For
        example, if we create `tests/common.rs` and place a function
        named `setup` in it, we can add some code to `setup` that we
        want to call from multiple test functions in multiple test
        files:
        When we run the tests again, we'll see a new section in the
        test output for the `common.rs` file, even though this file
        doesn't contain any test functions nor did we call the `setup`
        function from anywhere:

        Having `common` appear in the test results with 
        `running 0 tests`
        displayed for it is not what we wanted. We just wanted to share
        some code with the other integration test files.

        To avoid having `common` appear in the test output, instead of
        creating `test/common.rs`, we'll create 
        `tests/command/mod.rs`. This is an alternate naming convention
        that Rust also understands. Naming the file this way tells Rust
        not to treat the `common` module as an integration test 
        file. When we move the `setup` function code into
        `tests/common/mod.rs` and delete the `tests/common.rs` file, the
        section in the test output will no longer appear. Files in
        subdirectories of the _tests_ directory don't get compiled as
        separate crates or have sections in the test output.

        After we've created `tests/common/mod.rs`, we can use it from
        any of the integration test files as a module. We can add the
        `setup` function to the `it_adds_two` test in
        `tests/integration_test.rs`. Note that the `mod common;` 
        declaration is the same as the module declaration.Then in the
        test function, we can call the `common:setup()` function.

        Integration Tests for Binary Crates
        If our project is a binary crate that only contains a
        `src/main.rs` file and doesn't have a `src/lib.rs` file, we can't
        create integration tests in the _tests_ directory and bring
        functions defined in the `src/main.rs` file into scope with a
        `use` statement. Only library crates expose functions that other
        crates can use; binary crates are meant to be run on their own.

        This is one of the reasons Rust projects that provide a binary
        have a straightforward `src/main.rs` file that calls logic that
        lives in the `src/lib.rs` file. Using that structure, integration
        tests _can_ test the library crate with `use` to make the
        important functionality available. If the important functionality
        works, the small amount of code in the `src/main.rs` file will
        work as well, and that small amount of code doesn't need to be
        tested.

        Summary:
        Rust's testing features provide a way to specify how code should
        function to ensure it continues to work as you expect, even as
        you make changes. Unit tests exercise different parts of a
        library separately and can test private implementation details.
        Integration tests check that many parts of the library work
        together correctly, and they use the library's public API to
        test the code in the same way external code will use it. Even
        though Rust's type system and ownership rules help prevent some
        kinds of bugs, tests are still important to reduce logic bugs
        having to do with how your code is expected to behave.
    */
}
