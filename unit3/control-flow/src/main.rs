fn main() {
    let number = 3;

    if number < 5 {
        println!("The condition was true");
    }
    else {
        println!("The condition was false");
    }
    
    // This is how you do conditionals
    // conditionals "branch" into "arms" of code
    // The if conditional MUST be a bool. 

    if number != 0 {
        println!("The number was something other than 0");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let x = if number > 3 { 10} else {2};

    println!("The value of x is: {}", x);

    // assign values with an if statement.

    //let x = if condition { 5 } else { "six" };
    // this does not work because all possible values must be of the same type, compile-time

    // loop {
    //     println!("again!");
    // }

    //looping goes on forever until a break

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter >= 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // use break to return values from a loop
    // break is used as a statement always.

    let mut countdown = 3;

    while countdown != 0 {
        println!("{}!", countdown);

        countdown -= 1;
    }

    println!("LIFTOFF!!!");

    // while loops are loops with conditions to break

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("while loop: the value is: {}", a[index]);

        index += 1;
    }

    // loop through a collection with a while loop

    for element in a.iter() {
        println!("for loop: The value is {}", element);
    }

    // loop through a collection with a for loop
    // .iter() returns an iterator for the collection
    // remember that . allows you to access properties of a varaible, 
    // different from :: which lets you access parts of a package

    for number in (1..4).rev() {
        println!("for {}!", number);
    }
    println!("for LIFTOFF!!!");

    // loop through a range of numbers with a for loop
    // .rev() reverses the iterable
}
