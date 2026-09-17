// correct case

struct User<'a> {
    active: bool,
    username: &'a str,
    email: &'a str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active : true,
        username : "Hello",
        email : "hi@email.com",
        sign_in_count: 5
    };
}