fn main() {

    // This is an example of a literal (string lowercase s) coming into scope and going out of scope 
    {                       // s is not valid here, it's not yet declared
        let s = "hello";    // s is valid from this point forward

        // do stuff with s
    }                       // this scope is now over, and s is no longer valid

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print 'hello, world!'

    /*
        A string literal is constant so we know the size at compile time, so the text is
        hardcoded directly into the final executable. This makes string literals fast and 
        efficient.
        
        A String (capital S) is mutable which requires the data being taken from the heap because
        we can't know the size of the String at compile time because it could change. This makes
        Strings slower and less efficient than string literals.
    */

    {
        let s = String::from("hello");  // s is valid from this point forward

        // do stuff with s
    }                                   // this scope is now over, and s is no longer valid

    /*
        When a variable goes out of scope, Rust calls a special function for us. This function is
        called 'drop', and it's where the author of 'String' can put the code to return the memory. Rust
        calls 'drop' automatically at the closing curly bracket
    */



    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    /*
        In the above code x is created with the value of 5. Then, the variable y comes into scope and
        it has the same value as the value associated with the variable x. 
        "bind the value 5 to x; then make a copy of the value in x and bind it to y."
    */

    let s1 = String::from("hello");
    let s2 = s1;
    /*
        This looks similar to the above code with 'x' and 'y', but this isn't what happens. The stack
        value is copied but the stack value is a pointer to the heap so both s1 and s2 point to the
        same location on the heap so they both point to the same data. To prevent an error by calling
        drop for both s1 and s2 and free the same memory, the program consideres s1 invalid now. Trying
        to use s1 now would result in an error because Rust prevents you from using the invalid reference.
    */

    // If we want to do a deep copy we can use a common method called 'clone'
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    /*
        We don't need to use .clone() to make a copy for the integer case because integers are a
        fixed size and are placed on the Stack. The clone method is only used to make a deep copy of
        data that is on the heap (size not known at compile time)
    */

    /*
        The semantics for passing a value to a function are similar to those for assigning a value to a
        variable. Passing a variable that has data on the stack will copy, passing a variable that has
        data on the heap will copy the pointer to the data on the heap.
    */

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward


    // Returning values from functions can also transfer ownership.
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3

    /*
        With just this we would have to return any variable we pass into a
        function that we want to use again along with any values we wish to 
        be returned by the function. Fortunately, Rust has something for 
        this, references. 
    */
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);
    /*
        The ampersands (&) are references, and they allow you to refer to
        some value without taking ownership of it. The opposite of referencing by
        using '&' is dereferencing, which is accomplished with the dereference
        operator, '*'. By passing a reference to a function the function can
        access the information but does not own it and therefore the data will
        not be dropped when the function ends. Having a reference as
        function parameters is called borrowing. Trying to modify something that
        is borrowed will result in an error unless the reference is a
        mutable reference
    */
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
    /*
        Mutable references have one big restriction: you can have only
        one mutable reference to a particular piece of data in a
        particular scope. Many Rustaceans apparently struggle with this because
        most languages let you mutate whenever you'd like. The benefit of
        this is that it prevents data races. We can, however, use curly
        braces to create a new scope, allowing for multiple mutable
        references, just not simultaneous ones
    */
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    }   // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
    /*
        A similar rule exists for combining mutable and immutable references. You
        can have as many immutable references as you'd like, but if there is
        a mutable reference it must be the only reference, mutable or not.

        Note that a reference's scope starts from where it is introduced and
        continues through the last time that reference is used. For
        instance, this code will compile because the last usage of the
        immutable references occurs before the mutable reference is introduced.
    */

    let mut s = String::from("hello");
    let r1 = &s;        // no problem
    let r2 = &s;        // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point
    let r3 = &mut s;    // no problem
    println!("{}", r3);

    /*
        In languages with pointers, it's easy to erroneously create a
        dangling pointer, a pointer that references a location in memory 
        that may have been given to someone else, by freeing some memory
        while preserving a pointer to that memory. In Rust, by contrast,
        the compiler guarantees that references will never be
        dangling references
    */




}

fn takes_ownership(some_string: String) {   // some_string comes into scope
    println!("{}", some_string);
}   // Here, some_string goes out of sceop and 'drop' is called. The backing
    // memory is freed.

fn makes_copy(some_integer: i32) {  // some_integer comes into scope
    println!("{}", some_integer);
}   // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {                // gives_ownership will move its
                                                // return value into the function
                                                // that calls it

    let some_string = String::from("hello");    // some_string comes into scope

    some_string                                 // some_string is returned and
                                                // moves out to the calling
                                                // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String {   // a_string comes into
                                                        // scope

    a_string    // a_string is returned and moves out to the calling function
}

fn calculate_length(s: &String) -> usize {  // s is a reference to a String
    s.len()
}   // Here, s goes out of scope. But because it does not have ownership of what
    // it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}