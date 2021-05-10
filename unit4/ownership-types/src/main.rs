fn main() {
    //Each value in Rust has a variable that’s called its owner.
    //There can only be one owner at a time.
    //When the owner goes out of scope, the value will be dropped.  
    let _a = "hello"; //a is always valid
    if true {
        let _b = "hello"; //b is only valid in the scope
    }  

    // Rust calls a special function, "drop", to clean up data that is no longer valid. 
    // Rust called drop on _b at the closing curly brace.

    //String is a special data type; it is mutable unlike string literals (&str).
    //Unlike literals, it is put on the heap rather than the stack.

    let mut c = String::from("hello");
    // :: means that the function "from" is namespaced under "String"

    c.push_str(", world!");

    println!("{}", c); //prints "hello, world!"

    let s1 = String::from("hello");
    let s2 = s1;
    
    // "hello" is on the heap, so s1 and s2 are pointers, 
    // but this is not a "deep" or "shallow" copy. Instead,
    // Rust completely invalidated s1 and "moves" the pointer to s2.

    // This would fail because s1 is invalid:
    //  println!("{}, world!", s1);
    println!("{}, world!", s2);

    //Rust will NEVER make a deep clone automatically. Any 
    // automatic cloning is shallow.

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // .clone() is the proper way to make a deep copy in Rust.
    
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // If .clone() needs to be called, why is this valid?
    // This is NOT because integers are "primitives" or something
    // Rust gives the "Copy" trait to types that have a known size at compile time
    // This designates that .clone() would have no impact, and that variables don't get invalidated.
    // Types that have "Copy" include ints, bools, chars, floats, and tuples that contain such types
}
