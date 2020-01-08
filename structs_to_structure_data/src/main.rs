struct User {
    // define the names and types of the pieces of
    // data, which are called fields
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Tuple Structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// This is an impl block
// we can have multiple if we want
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    /*
        Using a struct we've define is creating an 
        instance of that struct. We create an
        instance by stating the name of the struct and
        then add curly brackets containing key: value 
        pairs, where the keys are the names of the
        fields and the values are the data we want to
        store in those fields. We don't have to specify
        the fields in the same order
    */
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    /*
        To get a specific value from a struct, we can use
        dot notation. We can also change a value by
        using the dot notation if the instance is
        mutable. The entire instance must be mutable, rust
        does not allow for specific fields to be
        mutable or immutable
    */
    let email = user1.email;
    println!("email is: {}", email);
    user1.email = String::from("anotheremail@example.com");
    println!("new email is: {}", user1.email);
    /*
        You can also return a struct from a function
    */
    let otherUser = build_user(String::from("example@another.com"), String::from("User123"));
    println!("{}", otherUser.username);
    /*
        If we want to create a struct using some of the same
        information as another struct, we can use the following
    */
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1 // this assigns the remaining fields to be the same as user1
    };

    /*
        You can define structs that look like tuples, called tuple structs. Tuple
        structs have the added meaning the struct name provides but don't have
        names associated with their fields, just types. They are useful when you
        want to give the whole tuple a name and make the tuple be a different
        type from other tuples.
    */
    let black = Color(0,0,0);
    let origin = Point(0,0,0);
    /*
        Both black and origin are different tuple structs.

        You can also define structs that don't have any fields! These
        are called unit-like structs because they behave similarly
        to (), the unit type.
    */
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1 is {:?}", rect1);
    println!("The area of the rectangle is {} square pixels.", area_func(&rect1));

    /*
        We can impement methods
    */
    println!("The area of the rectangle is {} square pixels.", rect1.area());
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    /*
        Another useful feature of impl blocks is that we can 
        define functions within impl blocks that don't take self as 
        a parameter. These are called associated functions because
        they're associated with the struct. They're still
        function, not methods, because they don't have an instance of
        the struct to work with.

        Associated functions are often used for constructors that will
        return a new instance of the struct.
    */
}

fn build_user(email: String, username: String) -> User {
    /*
        If we have a variable with the same name as the
        field we wish to assign it to, we can just put
        the variable as shown below. This is the
        field init shorthand syntax
    */
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn area_func(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}