fn main() {
    //Rust is statically typed!!!
    //Does not work because the compiler can't infer a type:
    // let guess = "42".parse().expect("Error: NaN");
    
    let _guess: u32 = "42".parse().expect("Error: NaN");

    // "Scalar" types are single values; ints, floats, bools, chars

    // Int: Rust defaults to i32, otherwise most common int type is u32,
    // you can use suffixes (57u8) or visual separation (1_000_000)
    // you can express as Hex (0xff), Octal (0o77), 
    // Binary (0b1111_1111), or Byte for u8 (b'A'). Overflow is possible!

    // Float: Rust defaults to f64. 

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    println!("x is an f64: {}, and y is an f32: {}", x, y);
    
    let sum = x + y;
    let difference = x - y; 
    let product = x * y;
    let quotient = x / y;
    let modulo = x % y;
    
    println!("operating: {}, {}, {}, {}, {}", 
	sum, difference, product, quotient, modulo);
    
    let t = true;
    let c = 'b';
    
    println!("our boolean is {} and our char is {}", t, c);

    // tuples are fixed in length once declared,
    // they can have different types for each element
   
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // gotta print it like this apparently
    println!("Here is a tuple: {:?}", tup);

    // extract values like this:

    let tup = (1, 2, 3);
    let (x, y, z) = tup;
    println!("Our values are {}, {}, and {}", x, y, z);

    // or this:
    
    println!("The second value of the tuple is: {}", tup.1);

    // Arrays are here, and they have fixed length and typing
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [5, 4, 3, 2, 1];
    println!("Here are some arrays: {:?}, {:?}", a, b);
    
    // shortcut: this initializes a 5 length array with all values as 3
    let a = [3; 5];
    println!("Shortcut array: {:?}", a);

    // accessing array elements is normal
    println!("Third element of b: {}", b[2]);

    // index OOB is caught by Rust in runtime. 
    // this wouldn't work
    // println!("Seventh element of b: {}", b[6]);
}
