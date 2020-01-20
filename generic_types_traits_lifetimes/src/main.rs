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