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





    /* COMMENTS */
    // This is a single line comment
    /*
    This is a multilined
    comment
    */





    /* CONTROL FLOW */
}
