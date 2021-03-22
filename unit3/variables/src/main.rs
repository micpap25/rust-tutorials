//constant values can be set in global scope
// underscores can be put in numbers for readibility

const MAX_POINTS: u32 = 100_000;

fn main() {
    // let x = 5;
    // wouldn't work because x wouldn't be mutable

    // mutating example
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of mutated x is: {}", x);

    println!("The max points constant is: {}", MAX_POINTS);

    // shadowing example
    let y = 5;
    println!("The value of y is: {}", y);

    let y = y + 1;
    println!("The value of shadowing y is: {}", y);

    // shadowing vs mutating

    // will not work, because spaces is still a string
    // let mut spaces = "   ";
    // spaces = spaces.len();

    // will work, because spaces is now a new variable
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The number of spaces is: {}", spaces);
    
}
