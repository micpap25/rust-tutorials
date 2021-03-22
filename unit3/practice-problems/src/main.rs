fn main() {
    convert_f_c(32, "f");
    convert_f_c(100, "c");
    for i in 3..30 {
        println!("Fib {} is {}", i, fib(i));
    }
    christmas();
}

fn convert_f_c(temp: i32, mode: &str) {
    let conv = if mode.eq("f") {
        ((temp - 32) * 5) / 9
    } else {
        ((temp * 9) / 5) + 32
    };

    if mode.eq("f") {
        println!("{} degrees Fahrenheit is {} degrees Celsius", temp, conv);
    } else {
        println!("{} degrees Celsius is {} degrees Fahrenheit", temp, conv);
    }
}

fn fib(num: i32) -> i32{
    if num == 1 || num == 2{
        return 1;
    } else {
        return fib(num - 1) + fib(num - 2)
    };
}

fn christmas() {
    let days = ["first", "second", "third", 
    "fourth", "fifth", "sixth", 
    "seventh", "eighth", "ninth", 
    "tenth", "eleventh", "twelfth"];
    let lyrics = ["a partridge in a pear tree.\n\n", 
    "two turtle doves,\nand ",
    "three French hens,\n", 
    "four calling birds,\n", 
    "five golden rings,\n", 
    "six geese a laying,\n", 
    "seven swans a swimming,\n", 
    "eight maids a milking,\n", 
    "nine ladies dancing,\n", 
    "ten lords a leaping,\n", 
    "eleven pipers piping,\n", 
    "twelve drummers drumming,\n"];

    for i in 0..12 {
        println!("On the {} day of Christmas, my true love gave to me,", days[i]);
        for j in (0..i+1).rev() {
            print!("{}", lyrics[j]);
        }
    }
}
