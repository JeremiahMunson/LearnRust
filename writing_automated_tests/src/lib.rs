#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn can_hold_err(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting_nameless(name: &str) -> String {
    String::from("Hello!")
}

pub fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                    value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                    value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2+2,4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        /*
            The `assert!` macro, provided by the standard library, is
            useful when you want to ensure that some condition in a
            test evaluates to `true`. We give the `assert!` macro an
            argument that evaluates to a boolean. If the value is
            `true`, `assert!` does nothing and the test passes. If
            the value is `false`, the `assert!` macro calls the
            `panic!` macro, which causes the test to fail.
        */

        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger () {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(smaller.can_hold(&larger) == false);
        assert!(!smaller.can_hold(&larger)); // !bool same as not the bool
    }

    #[test]
    fn larger_holds_smaller_err () {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(larger.can_hold_err(&smaller));
    }

    #[test]
    fn smaller_not_holds_larger_err () {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(!smaller.can_hold_err(&larger));
    }





    #[test]
    fn it_adds_two() {
        /*
            A common way to test functionality is to compare the
            result of the code under test to the value you expect the
            code to return to make sure they're equal. You can do this
            with the `assert!` macro using the `==` operator. However,
            this is so common that the std library provides a pair of
            macros - `assert_eq!` and `assert_ne!` - to perform this
            test more convenitently. They check to make sure the two
            input arguments are equal or not equal respectively. They
            also pring the two values if the assertion fails.

            Under the surface, these macros use the `==` and `!=`
            operators. The values being compared must implement the
            `PartialEq` and `Debug` traits. All primitive types have
            these implemented, but structs and enums you define will
            need to have `PartialEq` implemented to compare the
            values and `Debug` implemented to print the values if
            they fail. This is usually as straightforward as adding 
            the #[derive(PartialEq, Debug)] annotation to your 
            struct or enum definition.
        */
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn it_adds_three () {
        assert_eq!(5, add_two(2));
        /*
            ---- tests::it_adds_three stdout ----
            thread 'tests::it_adds_three' panicked at 'assertion failed: `(left == right)`
              left: `5`,
             right: `4`', src/lib.rs:100:9
        */
    }

    #[test]
    fn greeting_contains_name() {
        /*
            We can add custom messages to be printed with the failure
            message as optional arguments to the `assert!`, `assert_eq!`,
            and `assert_ne!` macros. Any arguments specified after the
            one required argument to `assert!` or the two required
            arguments to `assert_eq!` and `assert_ne!` are passed along
            to the `format!` macro. 
        */
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greetings did not contain name, value was '{}'", result
        );
    }

    #[test]
    fn greeting_without_name_contains_name () {
        let result = greeting_nameless("Carol");
        assert!(
            result.contains("Carol"),
            "Greetings did not contain name, value was '{}'", result
        );

        /*
            ---- tests::greeting_without_name_contains_name stdout ----
            thread 'tests::greeting_without_name_contains_name' panicked at 'Greetings did not contain name, value was 'Hello!'', src/lib.rs:139:9
        */
    }


    /*
        In addition to checking that our code returns the correct
        values we expect, it's also important to check that our code
        handles error conditions as we expect. For example, consider the
        `Guess` type that we created in Chapter 9. Other code that uses
        `Guess` depends on the guarantee that `Guess` instances will
        contain only values between 1 and 100. We can write a test that
        ensures that attempting to create a `Guess` instance with a
        value outside that range panics.

        We do this by adding another attribute, `should_panic`, to our
        test function. This attribute makes a test pass if the code inside
        the function panics; the test will fail if the code inside the
        function doesn't panic.

        Tests that use `should_panic` can be imprecise because they only
        indicate that the code has caused some panic. A `should_panic`
        test would pass even if the test panics for a different reason
        from the one we were expecting to happen. To make `should_panic`
        tests more precise, we can add an optional `expected`
        parameter to the `should_panic` attribute. The test harness will
        make sure that the failure message contains the provided
        text. 
    */
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100 () {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "Guess value must be greater than or equal to 1")]
    fn less_than_1 () {
        Guess::new(0);
    }

    #[test]
    #[should_panic]
    fn in_range_string () {
        Guess::new(50);
    }

    #[test]
    #[should_panic(expected = "Guess value must be greater than or equal to 1")]
    fn failing_panic () {
        Guess::new(200);
        /*
            ---- tests::failing_panic stdout ----
            thread 'tests::failing_panic' panicked at 'Guess value must be less than or equal to 100, got 200.', src/lib.rs:39:13
            note: panic did not include expected string 'Guess value must be greater than or equal to 1'
        */
    }

    /*
        So far we've written tests that panic when they fail. We can
        also write tests that use `Result<T, E>`. The `it_works`
        function now has a return type, `Result<(), String>`. In the
        body of the function, rather than calling the `assert_eq!`
        macro, we return `Ok(())` when the test passes and an `Err` with
        a `String` inside when the test fails.

        The below function now has a return type, 
        `Result<(), String>`. In the body of the function, rather than
        calling the `assert_eq!` macro, we return `Ok(())` when the test
        passes and an `Err` with a `String` inside when the test fails.

        Writing tests so they return a `Result<T,E>` enables you to use
        the question mark operator in the body of tests, which can be a
        convenient way to write tests that should fail if any
        operation within them returns an `Err` variant.

        You can't use the `#[should_panic]` annotation on tests that use
        `Result<T,E>`. Instead, you should return an `Err` value
        directly when the test should fail.
    */
    #[test]
    fn it_works_res() -> Result<(), String> {
        if 2+2 ==4  {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn it_doesnt_work() -> Result<(), String> {
        if 2+2 == 5 {
            Ok(())
        } else {
            Err(String::from("two plus two does not eqaul five"))
        }
    }
}

