fn main() {
    another_function(5, 6);
    // call functions like this
    expression_function();
    five();
    let z = five() - 1;
    println!("The value of z is: {}", z);
    let a = plus_three(z);
    println!("The value of a is: {}", a);
}

fn another_function(x: i32, y: i32) {
    // functions are defined with fn
    // functions can be given parameters as such

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// you can define functions anywhere, before or after main

// Function bodies are made up of a series of statements optionally ending in an expression.
// Because Rust is an expression-based language, this is an important distinction to understand.

// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resulting value. Let’s look at some examples.

// Statements don't return values, so you can't do something like this

// fn main() {
//     let x = (let y = 6);
// }

//Here is an example where you define a variable using a statement and expression
fn expression_function() {
    let x;

    let k = {
        x = 3;
        x + 1
    };

    println!("The value of k is: {}", k);
}
// note that expressions become statements because of the semicolon (think matlab)

fn five() -> i32 {
    5
}
// functions use -> to define a return type

fn plus_three(x: i32) -> i32 {
    x + 3
}
