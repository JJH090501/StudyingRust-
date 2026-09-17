#[derive(Debug)]

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("yongin"),
        email: String::from("yong1212@gmail.com"),
        sign_in_count: 1
    };

    let user2 = User {
        active: true,
        ..user1
    };

    println!("{user1:?}");
    println!("{user2:?}");
}

