#[derive(Debug)]
enum Option<T> {
    None,
    Some(T),
}

fn main() {
    let x: i32 = 5;
    let myValue: Option<i32> = Option::Some(x);
    println!("{myValue:?}")

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
}

// i8 != Option<i8>