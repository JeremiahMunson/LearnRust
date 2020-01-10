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



    /*
        Lastly, we'll cover the _hash map_. The type 'HashMap<K,V>' stores a
        mapping of keys of type 'K' to values of type 'V'. It does this via a
        _hashing function_, which determines how it places these keys and values
        into memory. Many programming languages support this kind of data
        structure, but they often use a different name: hash, map, object, hash
        table, dictionary, or associative array to name a few.

        Hash maps are useful when you want to look up data not by using an
        index, as you can with vectors, but by using a key that can be of any
        type. For example, in a game, you could keep track of each team's score
        in a hash map in which each key is a team's name and the values are each
        team's score. Given a team name, you can retrieve its score.

        You can create an empty hash map with 'new' and add elements with
        'insert'. In the code below, we're keeping track of the scores of two
        teams whose names are Blue and Yellow. The Blue team starts with 10 
        points, and the Yellow team starts with 50.
    */
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    /*
        Note we need to bring HashMap into scope with 'use'. Of the three
        collections covered here, HashMap is the least often used and have less
        support from the standard library; there's no built-in macro to construct
        them, for example.

        This 'HashMap' has keys of type 'String' and values of type 'i32'. Like
        vectors, hash maps are homogeneous: all of the keys must have the same
        type, and all of the values must have the same type.

        Another way of constructing a hash map is by using the 'collect' method on
        a vector of tuples, where eac tuple consists of a key and its value. The
        'collect' method gathers data into a number of collection types, including
        'HashMap'. For example, if we had the team names and initial scores in two
        separate vectors, we could use the zip method to create a vector of
        tuples where "Blue" is paired with 10, and so forth. Then we could use the
        'collect' method to turn that vector of tuples into a hash map, as shown
        below.
    */
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    /*
        The type annotation 'HashMap<_, _>' is needed here because it's possible
        to 'collect' into many different data structures and Rust doesn't know
        which you want unless you specify. For the parameters for the key and
        value types, however, we used underscores, and Rust can infer the 
        types that the hash map contains based on the types of the data in the
        vectors.

        For types that implement the 'Copy' trait, like 'i32', the values are
        copied into the hash map. For owned values like 'String', the values will
        be moved and the hash map will be the owner of those values, as
        demonstrated below.
    */
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point
    /*
        If we insert references to values into the hash map, the values won’t be 
        moved into the hash map. The values that the references point to must be 
        valid for at least as long as the hash map is valid. We’ll talk more about
        these issues in Chapter 10.

        We can get a value out of the hash map by providing its key to the
        'get' method, as shown below.
    */
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(i) => println!("score is: {}", i),
        None => println!("No team with that name"),
    };
    /*
        Here, 'score' will have the value that's associated with the Blue team, and
        the result will be 'Some(&10)'. The result is wrapped in 'Some' because
        'get' returns an 'Option<&V>'; if there's no value for that key in the hash
        map, get will return 'None'.

        We can iterate over each key/value pair in a hash map in a similar manner as
        we do with vectors, using a 'for' loop:
    */
    for (key,value) in &scores {
        println!("{}: {}", key, value);
    }
    /*
        Although the number of keys and values is growable, each key can
        only have one value associated with it at a time. When you want
        to change the data in a hash map, you have to decide how to
        handle the case when a key already has a value assigned. You
        could replace the old value with the new value, completely 
        disregarding the old value. You could keep the old value and
        ignore the new value, only adding the new value if the key
        _doesn't_ already have a value. Or you could combine the old
        value and the new value. Let's look at how to do each of these.

        If we insert a key and a value into a hash map and then insert
        that same key with a different value, the value associated with
        that key will be replaced. Even though the code below calls
        'insert' twice, the hash map will only contain one key/value pair
        because we're inserting the value for the Blue team's key both
        times.
    */
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);
    /*
        It's common to check whether a particular key has a value and, if
        it doesn't, insert a value for it. Hash maps have a special API
        for this called 'entry' that takes the key you want to check as
        a parameter. The return value of the 'entry' method is an
        enum called 'Entry' that represents a value that might or might 
        not exist. Let's say we want to check whether the key for the
        Yellow team has a value associated with it. If it doesn't, we
        want to insert the value 50, and the same for the Blue 
        team. Using the 'entry' API, the code looks like...
    */
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);
    /*
        The 'or_insert' method on 'Entry' is defined to return a mutable
        reference to the value for the corresponding 'Entry' key if that
        key exists, and if not, inserts the parameter as the new
        value for this key and returns a mutable reference to the new
        value. This technique is much cleaner than writing the logic
        ourselves and, in addition, plays more nicely with the borrow
        checker.

        Another common use case for hash maps is to look up a key's 
        value and then update it based on the old value. For 
        instance, the code below counts how many times each word
        appears in some text. We use a hash map with the words as keys and
        increment the value to keep track of how many times we've seen
        that word. If it's the first time we've seen a word, we'll first
        insert the value 0.
    */

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
    /*
        The 'or_insert' method actually returns a mutable reference 
        ('&mut V') to the value for this key. Here we store that mutable
        reference in the count variable, so in order to assign to that
        value, we must first dereference 'count' using the asterisk.

        By default, 'HashMap' uses a "cryptographically strong" hashing
        function that can provide resistance to Denial of Service (DOS)
        attacks. This is not the fastest hashing algorithm avaiable, but
        the trade-off for better security that comes with the drop in
        performance is worth it.
    */
}