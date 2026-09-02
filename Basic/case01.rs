fn main() {
    println!("Hello, World!");
    println!("Hello, Lua!");

    let sum = 100;

    println!("a+b={}", sum);
    println!("a+b={sum}");

    let result = add(1, 2);
    println!("1+2={}", result);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

