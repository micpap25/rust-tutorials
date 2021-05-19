fn main() {
    // Instead of giving ownership to functions and having to needlessly return objects
    // that are accessed but not modified, we can instead pass a reference to our object.
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // References let you refer to objects without affecting their ownership.
    // The reference character is &, the dereference character is *

    // Passing a reference is referred to as borrowing.
    // Because ownership doesn't change, the object cannot be modified!
    let s2 = String::from("hello");

    change(&s2);

    // To fix this, we can pass the object as a mutable reference.
    let mut s3 = String::from("hello");

    mut_change(&mut s3);

    println!("{}", s2);

    // But, you can't have multiple mutable references at once!
    // This helps prevent data races.

    let r1 = &mut s3;
    // let r2 = &mut s;

    println!("{}", r1);

    // Different scopes, of course, are fine.

    let mut s4 = String::from("hello");

    {
        let _r3 = &mut s4;
    }

    let _r4 = &mut s4;

    // You can't make a mutable when you already have a mutable, for the above reasons.
    // You can't make an immutable when you have a mutable, because the value changing is a problem.
    // You can have any number of immutables.

    let mut s5 = String::from("hello");

    let r5 = &s5;
    let r6 = &s5;
    //let r7 = &mut s5;
    println!("{} and {}", r5, r6);

    // The below is fine, though, because the object's scope ends at its last usage.

    let r8 = &s5;
    let r9 = &s5;
    println!("{} and {}", r8, r9);
    let _r10 = &mut s5;

    // Dangling references (refences to values that no longer exist) are an issue!
    // Don't return references to objects that are going out of scope!
    // Return the object itself!

    let reference = dangle();

    println!("{}", reference);
}

fn calculate_length(s: &String) -> usize {
    // usize is u32 on 32 bit arch, u64 on 64 bit arch
    s.len()
}

fn change(some_string: &String) {
    // some_string.push_str(", world");
    // This line wouldn't compile, you can't change some_string here!

    println!("{}", some_string);
}

fn mut_change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
// dangle returns a reference to a String
//     let s = String::from("hello"); // s is a new String
//     &s // we return a reference to the String, s
// }
// Here, s goes out of scope, and is dropped. Its memory goes away.
// Danger!

fn dangle() -> String {
    let s = String::from("hello");
    s
}
