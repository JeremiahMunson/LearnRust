use std::fmt::Display;

struct Point<T> {
    x: T,
    y: T,
}

struct PointDouble<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, U> PointDouble<T, U> {
    fn mixup<V, W>(self, other: PointDouble<V, W>) -> PointDouble<T, W> {
        PointDouble {
            x: self.x,
            y: other.y,
        }
    }
}
/*
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/

pub trait Summary {
    fn summarize1(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
    fn summarize_author(&self) -> String {
        String::from("the author")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize1(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize1(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // lifetime annotation not necessary by elision rules because the
    // method has &self so the return lifetime is set to follow the
    // lifetime of the object (I think)
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    /*
        We'll start with writing a function that takes some list and
        finds the largest element in the list. If we want to use the
        function for both i32 types and char types we'd have to write
        two separate functions: largest_i32() and largest_char(). With
        generic types we can make just one function. The `largest`
        function won't work just by simply adding the generic
        types because not all types can do what we are asking it to
        do. This can be fixed by types which will be covered later.

        We can also define structs to use a generic type parameter in
        one or more fields using the `<>` syntax. The code below shows
        how to define a `Point<T>` struct to hold `x` and `y` coordinate
        values of any type.
    */
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    //let wont_work = Point { x: 5, y: 4.0 };
    /*
        To define a Point struct where x and y are both generics but
        could have different types, we can use multiple generic type
        parameters. For example, `PointDouble`
    */
    let will_work = PointDouble { x: 5, y: 4.0 };
    let also_works = PointDouble { x: 5, y: 10 };
    let works_too = PointDouble { x: 1.0, y: 4.0 };
    /*
        We can also define enums to hold generic data types in their
        variants. Let's take another look at the `Option<T>` enum that
        the standard library provides. Also, the `Result<T, E>` enum.

        We can implement methods on structs and enums and use generic
        types in their definitions, too. We've defined a method named
        `x`. We must declare `T` just after `impl` so we can use it to
        specify that we're implementing methods on the type 
        `Point<T>`. We can also implement methods only on 
        `Point<f32>` instances rather than on `Point<T>` instances with
        any generic type. 

        Generic type parameters in a struct definition aren't always
        the same as those you use in the struct's method
        signatures. For example, the `mixup` method in
        `PointDouble` takes another `Point` as a parameter, which
        might have different types from the `self` `PointDouble` we're
        calling `mixup` on.
    */
    println!("integer.x = {}", integer.x());
    println!("float.distance_from_origin() = {}", float.distance_from_origin());
    let p2 = PointDouble { x: "Hello", y: 'c' };
    let p3 = will_work.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    /*
        Rust implements generics in such a way that your code doesn't
        run any slower using generic types than it would with concrete
        types.
    */

    /*
        A trait tells the Rust compiler about functionality a particular
        type has and can share with other types. We can use traits to
        define shared behavior in an abstract way. We can use trait
        bounds to specify that a generic can be any type that has certain
        behavior.

        A type's behavior consists of the methods we can call on that
        type. Different types share the same behavior if we can call the
        same methods on all of those types. Trait definitions are a way
        to group method signatures together to define a set of 
        behaviors necessary to accomplish some purpose.
    */
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize1());
    /*
        Sometimes it's useful to have default behavior for some or all of
        the methods in a trait instead of requiring implementations for
        all methods on every type. Then, as we implement the trait on a
        particular type, we can keep or override each method's default
        behavior.
    */
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };
    println!("New article available! {}", article.summarize());
    println!("1 new tweet: {}", tweet.summarize());
    /*
        We can define functions using traits. For example, the function
        notify that takes as a parameter a type that has implemented the
        `Summary` type.
    */
    notify(&tweet);
    /*
        The `notify` function is actually shorthand for the trait bound
        syntax used in `notify_bound`
    */
    notify_bound(&tweet);
    /*
        We could also specify more than one trait using the `+` operator

        pub fn notify(item: impl Summary + Display)

        pub fn notify<T: Summary + Display>(item: T)

        Using too many trait bounds has its downsides. Each generic has
        its own trait bounds, so functions with multiple generic type
        parameters can contain lots of trait bound information between
        the function's name and its parameter list, making the function
        signature hard to read. For this reason, Rust has alternate
        syntax for specifying trait bounds inside a `where` clause after
        the function signature. So instead of

        fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {

        we can use a `where` clause, like this:

        fn some_function<T, U>(t: T, u: U) -> i32
            where T: Display + Clone,
                  U: Clone + Debug
        {
        
        We can also use the `impl Trait` syntax in the 
        return position to return a value of some type that implements a
        trait
    */
    let new_tweet = returns_summarizable();
    /*
        We can also conditionally implement a trait for any type that
        implements another trait. Implementations of a trait on any
        type that satisfies the trait bounds are called blanket
        implementations and are extensively used in the Rust standard
        library. For example, the standard library implements the
        `ToString` trait on any type that implements the `Display`
        trait. The `impl` block in the standard library looks similar to
        this code:

        impl<T: Display> ToString for T {
    */




    /*
        Lifetimes are basically how long a variable is in scope. This
        can be problematic sometimes. Take the `longest` function that
        includes lifetime annotations. The lifetime annotation uses the
        apostraphe (') followed immediately by a reference name, usually
        all lowercase and very short. Most people use the name 'a. If
        the lifetime annotation wasn't used, the code wouldn't
        compile because the compiler wouldn't know what the lifetime of
        the returned reference would be.

        The lifetimes only give the compiler a way to check that
        references last long enough. In the code below, `string1` lasts
        for the outer lifetime and `string2` lasts for the inner
        lifetime. The result also lasts for the inner lifetime so the
        code will work even though the input lifetimes are no the
        same. As long as both variables last as long or longer than the
        returned reference needs to be. 
    */
    {
        let string1 = String::from("long string is long");
        let result;
        {
            let string2 = String::from("xyz");
            result = longest(string1.as_str(), string2.as_str());
            println!("The longest string is: {}", result);
        }
        /*
            If we tried printing result out here, the code wouldn't 
            compile because result would be used (and therefore last
            longer) after `string2` would have no longer be in scope. Even
            though `string1` is being returned and that does last it
            doesn't matter because the compiler doesn't know that.
        */
    }
    /*
        We can also add references to structs using lifetime
        annotations. For example, the `ImportantExcerpt` structure has
        a field `part` that takes a string slice which is a reference. The
        lifetime annotation means an instance of `ImportantExcerpt` can't
        outlive the reference it holds in its `part` field. 
    */
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt{ part: first_sentence };
    /*
        We don't need reference annotations for the function from
        chapter 4

        fn first_word(s: &str) -> &str {

        because early Rust programmers were using lifetime annotations a
        lot and decided they could implement it into the compiler to
        infer the lifetimes in these situations. These are implemented by
        the lifetime elision rules.



        We can implement methods on a struct with lifetimes. We use the
        same syntax as that of generic type parameters.
    */

    /*
        There's a special lifetime we need to discuss called 'static which
        means that this reference can live for the entire duration of the
        program. All string literals have the 'static lifetime, which we
        can annotate as follows:
    */
    let s: &'static str = "I have a static lifetime.";
    /*
        Sometimes the compiler may recommend using the 'static lifetime in
        error messages. Often times, the problem is with something other
        than adding the 'static lifetime, like a dangling reference. Take
        a close look at the real problem without just adding 'static
    */
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/*
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
*/

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

pub fn notify(item: & impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_bound<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Generic type parameters, trait bounds, and lifetimes together
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
} 