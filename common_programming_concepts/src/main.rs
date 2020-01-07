fn main() {
    /* VARIABLES AND MUTABILITY */
    let x = 5;
    println!("The value of x is: {}", x);
    //x = 6;
    // This would be an error because x is immutable so it cannot be changed

    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is: {}", y);
    // We can change y here because when we declared y we said that it is a
    // mutable variable with 'mut'

    // Using 'let' by itself is similar to 'const' but there are some differences
    // You can't do 'const mut' because constants must ALWAYS BE IMMUTABLE
    // When declaring a constant it MUST be annotated (type declared)
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS = {}", MAX_POINTS);
    // The underscore does not change the number, it is just useful for visuallizing
    //  the number like a comma
    // 100_000 == 100000 == 1_0_0_0_0_0 == 1000_00 == any other similar variation


    // A variable can be shadowed by reinitiallizing it
    // This is NOT THE SAME AS BEING MUTABLE
    let x = 4;
    println!("x = {}", x);
    // You can use the current variable when shadowing
    let x = x+3;
    println!("x = {}", x);
    // The variable x which has a value of 4 is now shadowing the variable x which has a value of 5
    let x = "Hello";
    // The type of a variable can also change with shadowing
    println!("x = {}", x);
    let x = x.len();
    println!("x = {}", x);





    /* DATA TYPES */
    // Rust is a statically typed language, so it must know the types of all variables at compile time.
    // The compiler can usually infer what type is desired based on how the variable is used
    // If the variable could be one of multiple data types, a type annotation is required like this
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess = {}", guess);

    // Scalar types
    // Integers, floats, numeric operations, booleans, and characters

    // Compound Types
    // Tuples and arrays
    
    // Signed integers are i8, i16, i32, i64, i128, isize
    // Unsigned integers are u8, u16, u32, u64, u128, usize
    // The default integer is i32

    // An integer can also be represented in decimal, hex, octal, binary, or byte (u8 only for byte)
    let mut x: i64 = 5;
    println!("x = {}", x);
    x *= -1;
    println!("x = {}", x);

    // Floating point numbers are f32 or f64 for 32 bits or 64 bits in size
    // The default is f64
    let x = 2.0; //f64
    let y: f32 = 3.0; // f32
    println!("x = {}, y = {}", x, y);

    // Boolean
    let t = true;
    let f = false;
    println!("t variable = {}, f variable = {}", t, f);

    // Character Type
    // The char type is four bytes in size and represents a Unicode Scalar Value
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c = {}", c);
    println!("z = {}", z);
    println!("emoji = {}", heart_eyed_cat);


    // Tuple type
    let tup = (500, 6.4, '😻');
    let another_tup: (i32, f64, char) = (300, 4.6, 'ℤ');
    let (x,y,z) = tup;
    println!("The value of y is: {}", y);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let cat_emoji = tup.2;
    println!("five_hundred = {}, cat_emoji = {}", five_hundred, cat_emoji);


    // Array type
    let a = [1,2,3,4,5];
    let months = [  "January", "February", "March", "April", "May", "June", "July",
                    "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1,2,3,4,5];

    // This will create an array with 5 values of 3 in it
    let a = [3; 5];

    let a = [1,2,3,4,5];

    let first = a[0];
    let second = a[1];
    println!("First = {}, Second = {}", first, second);
    let index = 3;
    let element = a[index];
    println!("The value of element is: {}", element);





    /* FUNCTIONS */
    // functions start with fn to signify that they are functions
    // the function 'another_function' is declared below main
    another_function();

    // Functions can have parameters. When declaring the function you should
    // specify the parameter types because it is good for the compiler to use to
    // determine the type of a variable that gets passed to the function
    function_with_parameter(5, 6);

    // To return a value you must include '->' followed by the data type after the parameters
    // You don't have to actually type 'return' to return a value from a function, just don't use ';'
    // You can use it to return the value and end the function early, but it isn't necessary
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    let y = func_return(y);
    println!("The value of y is: {}", y);

   



    /* COMMENTS */
    // This is a single line comment
    /*
    This is a multilined
    comment
    */





    /* CONTROL FLOW */
    // If statements 
    let number = 3;
    // You don't need/shouldn't(?) add parenthesis around the condition
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    // You can also use else if
    if number%4 == 0 {
        println!("number is divisble by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // You can assign variables using if/else
    // The data types must be the same because the compiler must
    // know the variables type at compile time
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);


    // Loops
    // There are 3 kinds of loops 'loop', 'while', 'for'

    // The loop 'loop' is an infinite loop
    // You can assign variables with 'loop'
    let mut counter = 0;
    let result = loop {
        counter+=1;

        if counter==10 {
            break counter*2;
        }
    };
    println!("The result is: {}", result);

    // 'while' is run until a condition is met
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }
    println!("LIFTOFF!!!");

    // 'for' lets you loop through each item in a collection
    let a = [10,20,30,40,50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // 'for' can also loop through numbers
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

}

fn another_function() {
    println!("Another function.");
}

fn function_with_parameter(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn func_return(x: i32) -> i32 {
    x+1
}