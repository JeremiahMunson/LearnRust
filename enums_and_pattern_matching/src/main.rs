enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct Ipv4Addr { }

struct Ipv6Addr { }

enum IpAddrTwo {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}


enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("Calling message");
    }
}



#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}



fn main() {
    /*
        We'll look at a situation were an enum coul be useful and
        more appropriate than structs. Say we need to work with
        IP addresses. Currently, two major standards are used
        for IP addresses: version four and version six. These
        are the only possibilities for an IP address that this
        program will come across: we can enumerate all possible
        variants, which is where enumeration gets its name.

        Because an IP address can be either V4 or V6, but not
        both at the same time. This makes using an enum for
        IP addresses appropriate because enum values can only
        be one of its variants.
    */
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    /*
        Note that the variants of the enum are namespaced under
        its identifier, and we use double colon to separate the
        two. This is useful becauase now both values are of the
        same type: IpAddrKind. We can then, for instance, define
        a function that takes any IpAddrKind
    */
    route(four);
    route(six);
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    /*
        Another example is 'enum Message' which has four
        variants with different types: 'Quit' has no
        data associated with it at all, 'Move' includes
        an anonymous struct inside it, 'Write' includes a
        single String, 'ChangeColor' includes three i32 values.

        Like structs, with an enum we can define methods using
        'impl'.
    */
    let m = Message::Write(String::from("hello"));
    m.call();

    // The 'Option' Enum and Its Advantages Over Null Values
    /*
        The Option type is used in many places because it encodes
        the very common scenario in which a value could be
        something or it could be nothing. Rust doesn't have the
        null feature that many other languages have. Instead of
        having nulls, Rust uses Option which is implemented in
        the standard library similar to below

        enum Option<t> {
            Some(T),
            None,
        }

        The 'Option' enum is so useful that it's even included in
        the prelude; you don't need to bring it into
        scope explicitly. Also, so are its variants, you can use
        'Some' and 'None' directly without 'Option::' prefix.
    */
    // With the Some value we know that a value is present so the
    // type is known
    let some_number = Some(5);
    let some_string = Some("a string");
    // When we do None we need to tell Option what type it would be
    let absent_number: Option<i32> = None;


    /*
        Rust has an extremely powerful control flow operator called
        'match' that allows you to compare a value against a series of
        patterns and then execute code base on which pattern
        matches. A match starts with the keyword 'match' followed by
        an expression, like 'if' except an 'if' statement requires the
        expression return a boolean but for match it can be
        whatever. Next are the match arms. Each arm has two parts: a
        pattern and some code. The pattern and the code are separated
        by the '=>' operator.

        When the 'match' expression executes, it compares the resulting
        value against the pattern of each arm, in order. If a pattern
        matches the value, the code associated with that pattern is
        executed. We can have as many arms as we need. If you want to
        run multiple lines of code in a match arm, you can use curly
        brackets.
    */
    let penny = Coin::Penny;
    println!("Penny costs {} cent(s)", value_in_cents(penny));
    /*
        Another useful feature of match arms is that they can bind to
        the parts of the values that match the pattern. This is how we
        can extract values of of enum variants. As an example, we can
        add the UsState to Quarters because some quarters have special
        state designs.
    */
    let quarter = Coin::Quarter(UsState::Colorado);
    println!("Quarter costs {} cent(s)", value_in_cents(quarter));
    /*
        We can use 'match' with 'Option<T>'. Previously, we wanted to 
        get the inner 'T' value out of the 'Some' case when using
        'Option<T>'. Let's say we want to write a function that takes
        an 'Option<i32>' and, if there's a value inside, adds 1 to that
        value. If there isn't a value inside, the function should return
        the 'None' value and not attempt to perform any operations. 
    */
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    /*
        If no arm pattern matches with the expression, this is an
        error. When we don't want to list every possible value, we
        can use the special pattern '_' instead. The '_' pattern will
        match any value. By putting it at the end it catches any case
        we did not explicitly make.
    */
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
    /*
        The 'match' express can be a bit wordy in a situation in which
        we care about only one of the cases. For this situation, Rust 
        provides 'if left'. 
        
        Let's say we want to do something only if the variable 'Some' value
        is 3, but nothing else.

        "
        if let _pattern_ = _expression_ {
            _code_
        }
        "
    */
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    // This works the exact same as the match above
    if let Some(3) = some_u8_value {
        println!("three");
    }
    // We can also add an else expression
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Arizona);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }
}

fn route(ip_kind: IpAddrKind) { 
    println!("route");
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}