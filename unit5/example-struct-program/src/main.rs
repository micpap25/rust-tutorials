// Implements the trait Debug so debug printing works. 
#[derive(Debug)]
struct Rect {
    width: usize,
    height: usize,
}

fn main() {
    // Let's write a sample program to demonstrate structs, like one that calculates a rectangle's area.
    // We could do it without structs, but we'd have to pass 2 parameters to the function, and there's no way to tell they're related.

    let width = 10;
    let height = 20;
    let area1 = area_unorganized(width, height);
    println!("{}", area1);

    // Now let's do it with tuples. That provides more structure, but you have to remember which index is which!

    let rect = (width, height);
    let area2 = area_tuple(rect);
    println!("{}", area2);

    // Finally, let's do it with the struct we defined above.

    let struct_rect = Rect { width, height };
    let area3 = area_struct(&struct_rect);

    println!("{}", area3);

    // How can we print our struct_rect?
    // This will not work, because Rect doesn't implement the trait Display
    // println!("{}", struct_rect);
    // Instead, we add ":?" inside the {} to print it out in debug mode, which requires the Debug trait
    // We derive that trait above
    println!("{:?}", struct_rect);

    // We can make this prettier with a # between the : and the ?

    println!("{:#?}", struct_rect);

    // In many cases, if you need a defined Rust trait, you can derive it as shown above. 
}

// A non-struct way to calculate area
fn area_unorganized(width: usize, height: usize) -> usize {
    width * height
}

fn area_tuple(dimensions: (usize, usize)) -> usize {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rect) -> usize {
    rectangle.width * rectangle.height
}
