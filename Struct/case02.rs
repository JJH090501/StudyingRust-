#[derive(Debug)] // an attribute that allows print struct.
struct Player {
    level: i32,
    username: String,
    password: String,
}

fn newUser(username: String, password: String) -> Player {
    Player {
        level: 0,
        username: username,
        password: password
    }
}

fn main() {
    let user1 = newUser("yongin".to_string(), "ayongin".to_string());

    println!("{user1:?}");
}