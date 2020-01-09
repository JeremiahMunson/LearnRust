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
}

fn route(ip_kind: IpAddrKind) { 
    println!("route");
}
