pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// Absolute
//use crate::front_of_house::hosting;
// Relative
//use front_of_house::hosting;
// We could also do
//use crate::front_of_house::hosting::add_to_waitlist
// but by bringing in the parent we know that the
// function add_to_waitlist isn't locally defined

/*
    When we bring a name into scope with 'use', the name
    available in the new scope is private. To enable the
    code that calls our code to refer to that name as if
    it had been defined in that code's scope, we can
    combine 'pub' and 'use'. This technique is called
    "re-exporting" because we're bringing an item into
    scope but also making that item available for others to
    bring into their scope
*/
pub use crate::front_of_house::hosting;
/*
    By using 'pub use', external code can now call the
    'add_to_waitlist' function using 'hosting::add_to_waitlist'. If
    we hadn't specified 'pub use', the 'eat_at_restaurant' function 
    could call 'hosting::add_to_waitlist' in its scope, but external
    code couldn't take advantage of this new path.
*/

pub fn eat_at_resaurant() {
    // Absolute path
    //crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    //front_of_house::hosting::add_to_waitlist();

    // Because we brought the hosting path into scope we can just do
    hosting::add_to_waitlist();


    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn serve_order() {}

mod back_of_house {
    /*
        This function is in the back_of_house module, so we can use
        'super' at the start of the path to go to the parent module of
        'back_of_house' which in this case is 'crate' (the root). This 
        is like starting a filesystem path with the '..' syntax. This 
        is a relative path.
    */
    fn fix_incorrect_order () {
        cook_order();
        super::serve_order();
    }
    fn cook_order() {}



    /*
        Structs and enums can be public also. 
    */
    // Structs need the struct itself to be public, and then each
    // function and field default to private still
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    // Enums once the enum is public all of the fields are public too
    pub enum Appetizer {
        Soup,
        Salad,
    }

}