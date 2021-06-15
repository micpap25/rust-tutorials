#[derive(Debug)]
struct Rectangle {
    width: usize,
    height: usize,
}

// Methods are functions that are defined in the context of a Struct. They are very organized.
// Thier first parameter is always self, which represents the instance that the function is being called on.
// Start defining functions with the impl (implementation) block.

// Rather than something like rect: &Rectangle, Rust allows us to just put &self.
// Rust knows it is dealing with a Rectangle because of the impl block.
// You still need to reference self because it's a function that can borrow things.

// Use cases: Use &self if you just want to access data about the instance.
// Use &mut self if you want to write to the instance.
// There aren't many use cases for self, only if you want to change the instance permanently.
impl Rectangle {
    fn area(&self) -> usize {
        self.width * self.height
    }
    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        self.width > other_rect.width && self.height > other_rect.height
    }
    // Similar to methods are associated functions. 
    // They are not methods because they do not access self.
    // They might be useful features to classify under a struct.
    fn square(size: usize) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// If you want to, you can create multiple impl blocks for the same struct. This becomes important later.
impl Rectangle {
    fn perimiter(&self) -> usize {
        2 * self.width + 2 * self.height
    }
}

fn main() {
    // Define the structs
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // Call the method with method syntax, period after the instance.
    // Rust has automatic dereferencing; you don't have to derefernce instances when calling methods.
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Call an associated function with assocated function syntax, double colon after the name of the struct.
    // This is also used for namespaces of modules.
    let square = Rectangle::square(5);
    println!(
        "{}",
        square.perimiter()
    );
}