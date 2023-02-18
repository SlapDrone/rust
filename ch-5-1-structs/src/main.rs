// archetypical struct
// each instance owns all its data and that data is valid
// as long as entire struct is
// refs to data owned elsewhere possible with lifetimes (Ch10)
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple struct (no named fields but distinct types)
struct Colour(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like structs (useful to implement trait on)
// some type but no data to store in type
struct AlwaysEqual;


fn build_user(email: String, username: String) -> User {
    User{
        // field init shorthand
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername")
    );
    user1.email = String::from("actually@this.org");
    println!("Hello, {}!", user1.username);

    // struct update syntax
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // copies remaining attrs
    };
    println!("...and {} ({})!", user2.username, user2.email);
    // using user1 is now invalid as assingment of user2
    // has moved references to string fields (these don't
    // implement the Copy trait)
    //println!("{}", user1.username) // breaks

    // tuple structs
    let black = Colour(0, 0, 0);
    let origin = Point(0, 0, 0);

    // unit-like struct
    let subject = AlwaysEqual;
}
