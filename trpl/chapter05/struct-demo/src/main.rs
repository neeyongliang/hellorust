struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 0,
    }
}

fn main() {
    println!("Hello World!");

    let user1 = User {
        email: String::from("abc@gmail.com"),
        username: String::from("ABC"),
        active: true,
        sign_in_count: 556,
    };
    let use2 = build_user(String::from("apple"), String::from("apple@icloud.com"));
}
