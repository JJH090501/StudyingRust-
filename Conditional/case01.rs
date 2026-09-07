use std::io::stdin;

fn main() {
    let mut a = 5;
    let mut b = 9;

    if a + b > 10 {
        println!("you did it!");
    } 
    else if a + b == 7 {
        println!("ah naw");
    } else {
        println!("No way!");
    }
}