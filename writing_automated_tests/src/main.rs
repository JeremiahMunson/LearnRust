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
    */
}
