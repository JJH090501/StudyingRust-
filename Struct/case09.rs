#[derive(Debug)]

struct Person<'a> {
    hand: &'a str,
    leg: &'a str
}

fn main() {
    let person1 = Person {
        hand: "both",
        leg: "none"
    };

    dbg!(&person1);
}