use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  println!("Guess the number!"); 
  //prints intro

  let secret_number = rand::thread_rng().gen_range(1, 101);
  //thread_rng() creates a random number handler
  //gen_range(1, 101) makes a random number from 1 to 100.

  let mut num_guesses = 0;

  loop {
  //loop causes an infinite loop 

    println!("Please input your guess.");

    let mut guess = String::new(); //creates a mutable new String variable
    // "::" means an associated function
  
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    //stdin() returns an input handler, ".read_line" is a method of the handler
    //read_line takes input and puts it into its parameter. & means a reference.
    //read_line() returns a Result, which can be Ok or Err. 
    //expect() is a crasher and error handler if the Result is Err.

    num_guesses += 1;

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };
    //shadowing variables is allowed in Rust
    //trim() cleans it, parse() turns it into a number 
    //we tell it that it will make a u32 (unsigned 32 bit int)

    println!("You guessed: {}", guess);
    //{} is where the parameter goes. 
  
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      }
    }
    //match starts a decision with many arms 
    //Ordering has results for match
  }
  println!("You guessed {} times!", num_guesses);
}