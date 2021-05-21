// Structs are a method of organizing data.
// They can be compared to tuples, but instead of information having indexes to access them from,
// they have names, which make structs much more convinient.

// A struct is defined by using the struct keyword, and then listing a series of fields.
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Tuple structs are ways to create organized structures without naming individual fields

struct Color(i32, i32, i32);

// Lastly, unit structs are useful for type implementation

struct Unit();

fn main() {
    // After defining a struct, you can create instances of it in your code.
    // Each field is given data in a key: value format.
    // The fields do not have to be assigned in the order they were defined.

    let user1 = User {
        // As a reminder, presenting different ways to go 
        // from &str (pointer to string data in program) to String (heap-allocated string)
        username: String::from("micpap25"), 
        email: "micpap25@gmail.com".to_string(),
        active: false,
        sign_in_count: 37
    };

    println!("{} {}", user1.username, user1.email);
    // If the struct instance is mutable, you can also use dot notation to edit the values
    // You cannot make specific fields mutable; the whole instance is mutable or immutable

    let mut _user2 = User {
        username: String::from("micpap25"), 
        email: "micpap25@gmail.com".to_string(),
        active: false,
        sign_in_count: 38
    };

    _user2.sign_in_count = 39;

    // You can define build functions for structs

    let _user3 = build_user_bad("dude@gmail.com".to_string(), "dude".to_string());
    let _user4 = build_user_good("guy@gmail.com".to_string(), "dude".to_string());

    // We can use attributes from other instances to build a new instance

    let _user5 = User {
        username: String::from("user"),
        email: String::from("user@gmail.com"),
        active: user1.active,
        sign_in_count: user1.sign_in_count
    };

    // There's a shorthand way to do this, too
    // ..<instance name> fills in all unspecified values with the instance's values.

    let _user6 = User {
        active: true,
        sign_in_count: 2,
        ..user1
    };

    // Instantiate a tuple struct as so:

    let _black = Color(0, 0, 0);

    // And a unit struct:

    let _unit = Unit();

    // When defining a struct, it is simplest to use an owned type 
    // like String, rather than a referenced type like &str. 
    // It makes sense that the struct owns all its data. 
    // You could make a struct with references, 
    // but that requires using lifetimes (which we'll learn about later)
    // to assure the data's existence.

}

// This build function works but is unwieldy because the field names have to be repeated

fn build_user_bad(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// This build function user the field init shorcut to be cleaner

fn build_user_good(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}