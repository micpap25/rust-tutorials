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

fn main() {
    // After defining a struct, you can create instances of it in your code.
    // Each field is given data in a key: value format.
    // The fields do not have to be assigned in the order they were defined.

    let _user1 = User {
        // As a reminder, presenting different ways to go 
        // from &str (pointer to string data in program) to String (heap-allocated string)
        username: String::from("micpap25"), 
        email: "micpap25@gmail.com".to_string(),
        active: false,
        sign_in_count: 37
    };

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