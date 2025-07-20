struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = build_user(
        String::from("someusername123"),
        String::from("someone@example.com"),
    );

    let user2 = User {
        email: String::from("anotheremail@example.com"),
        ..user1
    };

    user1.username = String::from("differentusername");
    user1.email = String::from("evendifferentemail@example.com");

    print_user(user1);
    print_user(user2);
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn print_user(user: User) {
    println!("{} on {}", user.username, user.email);
}
