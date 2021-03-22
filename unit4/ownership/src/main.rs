fn main() {
    //Each value in Rust has a variable that’s called its owner.
    //There can only be one owner at a time.
    //When the owner goes out of scope, the value will be dropped.  
    let _a = "hello"; //a is always valid
    if true {
        let _b = "hello"; //b is only valid in the scope
    }  

    //String is a special data type; it is mutable unlike string literals (&str).
    //Unlike literals, it is put on the heap rather than the stack.

    let mut c = String::from("hello");
    // :: means that the function "from" is namespaced under "String"

    c.push_str(", world!");

    println!("{}", c); //prints "hello, world!"
}
