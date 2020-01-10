/*
    Rust's standard library includes a number of very useful
    data structures called _collection_. Unlike built-in
    array and tuple types, the data these collections point to 
    is stored on the heap. This means the amount of data does not
    need to be known at compile time and can grow or shrink as the
    program runs.

    https://doc.rust-lang.org/std/collections/index.html

    Sequences:  Vec, VecDeque, LinkedList
    Maps:       HashMap, BTreeMap
    Sets:       HashSet, BTreeSet
    Misc:       BinaryHeap
*/


enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


fn main() {
    // To create a new, empty vector, we can call the 'Vec::new' function
    let v: Vec<i32> = Vec::new();
    /*
        Note we added a type annotation here. Because we aren't inserting any
        values into the vector, Rust doesn't know what kind of elements we intend
        to store. This is an important point. Vectors are implemented using
        generics (covered in chapter 10) so Vec<T> can hold any type.

        In more realistic code, Rust can often infer the type of value you 
        want to store once you insert values, so you rarely need to do the type
        annotation. It's more common to create a Vec<T> that has initial
        values, and Rust provides the 'vec!' macro for convenience.

        Also, like any other 'struct', a vector is freed when it goes out of scope
    */
    {
        let v = vec![1, 2, 3];
        // do stuff with v
    }   // <- v goes out of scope and is freed here

    /*
        To create a vector and then add elements to it, we can use the
        'push' method as shown below.
    */
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    /*
        There are two ways to reference a value stored in a vector. In the
        exemples we've annotated the types of the values that are
        returned from these function for extra clarity.

        You can access a value in a vecor either with indexing syntax or
        the 'get' method.
    */
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    /*
        Note we used the index value of 2 to get the third element: vectors
        are indexed by number starting at zero. Second, the two ways to get
        the third element are by using & and [], which gives us a 
        reference, or by using the 'get' method with the index passed as an
        argument, which gives us an Option<&T>

        Also, if attempting to access an element at an index that the vector
        does not have, the '[]' method will cause the program to panic because
        it references a nonexistent element. This is best used when you want
        your program to crash if there's an attempt to access an element past
        the end of the vector.

        When the 'get' method is pased an index outside the vector, it returns
        'None' without pancking. You would use this if accessing an element
        beyond the range of the vector happens occasionally under normal
        circumstances.

        When the program has a valid reference, the borrow checker enforeces
        the ownership and borrowing rules to ensure this reference and any
        other references to the contents of the vector remain valid.

        This would result in an error...

        let mut v = vec![1, 2, 3, 4, 5];
        let first = &v[0];
        v.push(6);
        println!("The first element is: {}", first);

        This error comes from the way vectors work. Adding a new element onto
        the end of the vector might require allocating new memory and copying
        the old elements to the new space if there isn't enough room to put
        all the elements next to each other where the vector currently is.
    */

    /*
        If we want to access each element in a vector in turn, we can iterate
        through all the elements rather than use indices to access one at a time.
    */
    let mut v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    for i in &mut v {
        // To change the value the mutable reference refers to, we use the 
        // dereference operator '*' to get the value in 'i' before using '+='
        *i += 50;
        println!("{}", i);
    }

    /*
        Vectors can only store values that are the same type. This can be
        inconvenient; there are definitely use cases for needing to store
        a list of items of different types. Fortunately, the variants of an
        enum are defined under the same enum type, so when we need to store 
        elements of a different type in a vector, we can define and use an enum

        Example: Say we want to get values from a row in a spreadsheet in
        which some of the columns in the row contain integers, some
        floating-point numbers, and some strings. We can define an enum whose
        variants will be considered the same type: the enum. Then create a 
        vector that holds that enum and so, ultimately, holds different types.
    */
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    /*
        Rust needs to know what types will be in the vector at compile time so
        it knows exactly how much memory on the heap will be needed to store
        each element. A secondary advantage is that we can explicit about what
        types are allowed in this vector. If Rust allowed a vector to hold any
        type, there would be a change that one or more of the types would cause
        errors with the operations performed on the elements of the vector. Using
        an enum plus a 'match' expression means that Rust will ensure at compile
        time that every possible case is handled.
    */
    // THERE ARE MORE WAYS TO USE VECTORS SO CHECK THE API DOCUMENTATION


    /*
        New Rustaceans commonly get stuck on strings for a combination of three
        reasons: Rust's propensity for exposing possible errors, strings being a
        more complicated data structure than many programmers give them credit
        for, and UTF-8. These factors combine in a way that can seem difficult when 
        you’re coming from other programming languages.

        We discuss Strings in the context of collections because Strings are 
        implemented as a collection of bytes, plus some methods to provide useful
        functionality when those bytes are interpreted as text.

        Rust has only one string type in the core language, which is the string slice
        'str' that is usually seen in its borrowed form '&str'. The 'String' type, which
        is provided by Rust's standard library rather than coded into the core 
        language, is a growable, mutable, owned, UTF-8 encoded string type. When
        Rustaceans refer to "strings" in Rust, they usually mean the 'String' and the
        string slice '&str' types, not just one of those types. 

        Rust's standard library also includes a number of other string types, such as
        'OsString', 'OsStr', 'CString' and 'CStr'. See how all of those names all end
        in 'String' or 'Str'? They refer to owned and borrowed variants, just like the
        'String' and 'str' types seen previously. We won't discuss these other string
        types in this chapter; see their API docs for more about them.

        Many of the same operations available with 'Vec<T>' are available with
        'String' as well, starting with the 'new' function to create a string.
    */
    let mut s = String::new();
    /*
        This line creates a new empty string called s which we can then load data
        into. Often, we'll have some initial data that we want to start with. For
        that, we use the 'to_string' method, which is available on any type that
        implements the 'Display' trait, as string literals do.
    */
    let data = "initial contents";
    let s = data.to_string();
    // the mothod also works on a literal directly;
    let s = "initial contents".to_string();
    /*
        This code creates a string containing 'initial contents' (without the quotes). We
        can also use the function 'String::from' to create a String from a string
        literal. The code below is equivalent to the code above.
    */
    let s = String::from("initial contents");
    /*
        Because strings are used for so many things, we can use many different generic 
        APIs for strings, providing us with a lot of options. Some of them can seem 
        redundant, but they all have their place. Here, 'String::from' and 'to_string' do
        the same thing, so which you choose is a matter of style.

        Remember that strings are UTF-8 encoded, so we can include any properly encoded
        data in them, as shown below.
    */
    let hello1 = String::from("السلام عليكم");
    let hello2 = String::from("Dobrý den");
    let hello3 = String::from("Hello");
    let hello4 = String::from("שָׁלוֹם");
    let hello5 = String::from("नमस्ते");
    let hello6 = String::from("こんにちは");
    let hello7 = String::from("안녕하세요");
    let hello8 = String::from("你好");
    let hello9 = String::from("Olá");
    let hello10 = String::from("Здравствуйте");
    let hello11 = String::from("Hola");



    /*
        A 'String' can grow in size and its contents can change, just like the contents of
        'Vec<T>', if you push more data into it. In addition, you can conveniently use the
        '+' operator or the 'format!' macro to concatenate 'String' values.

        We can grow a 'String' by using the 'push_str' method to append a string slice.
    */
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s is: {}", s);
    /*
        The 'push_str' method takes a string slice because we don't necessarily want to take
        ownership of the parameter. For example, the code below shows that it would be
        unfortunate if we weren't able to use 's2' after appending its contents to 's1'
    */
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is: {}", s2);
    /*
        The 'push' method takes a single character as a parameter and adds it to the
        'String'. Remember, a character only has single quotes.
    */
    let mut s = String::from("lo");
    s.push('l');
    println!("s is: {}", s);

    /*
        Often you'll want to combine two existing strings. One way is to use the
        '+' operator
    */
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("s3 is: {}",s3);
    /*
        The reason 's1' is no longer valid after the addition and the reason we used a
        reference to 's2' has to do with the signature of the method that gets called when
        we use the '+' operator. The '+' operator uses the 'add' method, whose signature
        looks like this:

        fn add(self, s: &str) -> String {}
        
        This isn't exactly how it is implemented, but it's a good way of thinking like it now

        Also note that the function wants type '&str' but '&s2' is type '&String'. The compiler
        can coerce the '&String' argument into a '&str'. When we call the 'add' method, Rust 
        uses a _deref coercion_, which here turns '&s2' into '&s2[..]'. This will be discussed
        more in Chapter 15.

        We can also add multiple strings at once.
    */
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s is: {}", s);
    /*
        We can also use the 'format!' macro which easier to read and doesn't take ownership of
        any of its parameters.
    */
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);

    /*
        In many other programming languages, accessing individual characters in a 
        string by referencing them by index is a valid and common operation. However,
        if you try to access parts of a 'String' using indexing syntax in
        Rust, you'll get an error.

        Rust strings don't support indexing. A 'String' is a wrapper over a
        'Vec<u8>'. Let's look ast some of our properly encoded UTF-8 example
        strings from before.
    */
    let len = hello11.len();
    println!("{} has a length of {}", hello11, len);
    let len = hello10.len();
    println!("{} has a length of {}", hello10, len);
    /*
        The first, "Hola" has a length of 4, which means the vector storing the
        string "Hola" is 4 bytes long. Each of these letters takes 1 byte when
        encoded in UTF-8. But what about the next string? Asked how long the
        string is, you might say 12. However, Rust's answer is 24: that's the
        number of bytes it takes to encode "Здравствуйте" in UTF-8, because each
        Unicode scalar value in that string takes 2 bytes of storage. Therefore, an
        index into the string's bytes will not always correlate to a valid Unicode
        scalar value. To domonstrate, consider this invalid Rust code:

        let hello = "Здравствуйте";
        let answer = &hello[0];

        What should the value of 'answer' be? Should it be З, the first
        letter? when encoded in UTF-8, the first byte of З is 208 and the second
        is 151, so 'answer' should be 208, but 208 is not a valid character on
        its own. 
    */

    /*
        If we look at the Hindi word “नमस्ते” written in the Devanagari script, it is 
        stored as a vector of 'u8' values that looks like this:

        [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]

        That’s 18 bytes and is how computers ultimately store this data. If we look 
        at them as Unicode scalar values, which are what Rust’s char type is, those 
        bytes look like this:

        ['न', 'म', 'स', '्', 'त', 'े']

        There are six 'char' values here, but the fourth and sixth are not
        letters: they're diacritics that don't make sense on their own. Finally,
        if we look at them as grapheme clusters, we'd get what a person would
        call the four letters that make up the Hindi word:
        
        ["न", "म", "स्", "ते"]

        Rust provides different ways of interpreting the raw string data that 
        computers store so that each program can choose the interpretation it 
        needs, no matter what human language the data is in.


        A final reason Rust doesn't allow us to index into a String to get a
        character is that indexing operations are expected to always take constant
        time (O(1)). But it isn't possible to guarantee that performance with a
        'String', because Rust would have to walk through the contents from the 
        beginning to the index to determine how many valid characters there were.
    */

    /*
        Indexing into a string is often a bad idea because it's not clear what the
        return type of the string-indexing operation should be: a byte value, a
        character, a grapheme cluster, or a string slice. Therefore, Rust asks you
        to be more specific if you really need to use indices to create string
        slices. To be more specific in your indexing and indicate that you want a
        string slice, rather than indexing using '[]' with a single number, you
        can use '[]' with a range to create a string slice containing particular
        bytes:
    */
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    /*
        Here, 's' will be a '&str' that contains the first 4 bytes of the
        string. Earlier, we mentioned that each of these characters was 2 bytes,
        which means 's' will be Зд.

        What would happen if we used &hello[0..1]? The answer: Rust would
        panic at runtime in the same way as if an invalid index were accessed in
        a vector.
    */

    /*
        Fortunately, you can access elements in a string in other ways. If you
        need to perform operations on individual Unicode scalar values, the best
        way to do so is to use the 'chars' method. Calling 'chars' on “नमस्ते”
        separates out and returns six values of type 'char', and you can iterate
        over the result to access each element:
    */
    for c in hello5.chars() {
        println!("{}", c);
    }
    // The 'bytes' method returns each raw byte, which might be appropriate for
    // your domain:
    for b in hello5.bytes() {
        println!("{}", b);
    }
}

