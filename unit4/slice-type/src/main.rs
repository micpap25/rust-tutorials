fn main() {
    // The challenge; Write a function that gets the first word in a string.
    // We will accomplish this by finding the first space in the string. We can get the "slice" from there.
    let mut s = String::from("hello world");

    let _word = first_word_usize(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
    // imagine if we had this function: fn second_word(s: &String) -> (usize, usize)
    // think of all those variables that are floating around!
    // instead we can use slices.

    let sliced_string = String::from("hello world");

    // These are references to portions of the sliced_string
    // For example, world's pointer points to the 7th (counting from 1) byte of sliced_string
    let _hello = &sliced_string[0..5];
    let _world = &sliced_string[6..11];

    // Easy way to slice from the start of a string
    let _start_slice = &sliced_string[..2];
    // Easy way to slice to the end of a string
    let _end_slice = &sliced_string[3..];
    // Easy way to slice the whole string
    let _whole_slice = &sliced_string[..];
    // BTW, you can't slice through the middle of a non-UTF-8 character

    // Let's re-write first_word to work with slices
    let mut s2 = String::from("hello world");

    let word = first_word_slice(&s2);

    println!("the first word is: {}", word);

    s2.clear();

    // Slices are also nice because they warn us compile-time that a value was cleared, instead of run-time.
    
    // Now, we can understand string literals
    let _literal = "Hello, world!";
    // This is an immutable &str which references a specific point in memory.
    // Here's a guide: https://stackoverflow.com/questions/47179667/is-a-string-array-e-g-string-3-stored-on-the-stack-or-heap/47179941

    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let _word = first_word_best(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let _word = first_word_best(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_best(my_string_literal);

    println!("{}", word);

    // And of course, we can slice other types too. 
    // this example has type &[i32]
    let a = [1, 2, 3, 4, 5];

    let _slice = &a[1..3];

    // All these topics make it easier for Rust to control data in the backend.
}

fn first_word_usize(s: &String) -> usize {
    // To go through each byte of the string, we will use .as_bytes() to make it a byte array
    // Remember that bytes are u8's. 
    let bytes = s.as_bytes();

    // .iter() returns an iterator; for this purpose, equivalent to returning each element of the array in a way that can be altered.
    // .enumerate() takes the .iter() array and turns it into an array of tuples,
    // where the first item is the index and the second is a reference to the element at that index.
    // (i, &item) deconstructs the tuple
    for (i, &item) in bytes.iter().enumerate() {
        // if we find a space, return its index
        if item == b' ' {
            return i;
        }
    }
    // if we don't find a space, return the length of the string (the whole thing is one word)
    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// How can we make this best? return a string literal.
fn first_word_best(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // if we find a space, return its index
        if item == b' ' {
            return &s[0..i];
        }
    }
    // if we don't find a space, return the length of the string (the whole thing is one word)
    &s[..]
}