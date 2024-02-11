struct User {
    email: String,
    active: bool,
    sign_in_count: u32,
    username: String,
}

fn main() {
    let user1 = User {
        email: String::from("hello@example.com"),
        active: true,
        sign_in_count: 1,
        username: String::from("Lebron James"),
    };

    // --snip--
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("{}", user2.email);
}

