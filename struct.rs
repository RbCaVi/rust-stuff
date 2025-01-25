// definition
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// standard initialization
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}

// member assignment
// you cannot mark members mutable or not in the variable declaration
// just the whole struct
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}

// standard initialization
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// js style
// if the variable has the same name as the member
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// standard initialization
// partial copy
fn main() {
    // --snip--

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}

// "spread"
// assign some values explicitly and copy the rest from another instance
// they call it struct update
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}

// tuple structs
// basically a tuple
// different types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// unit like struct
// no fields
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}

// if you put references in an object
// then you have to understand lifetimes....