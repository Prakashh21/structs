fn main() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let mut user2: User = User {
        active: true,
        username: String::from("someusername456"),
        email: String::from("someoneelse@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    print!("{0}", user1.email)
}
