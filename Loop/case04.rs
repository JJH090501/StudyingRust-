fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the element is {element}");
    }

    for number in (1..4) {
        println!("{number}")
    }
    
    // rev() is reverse
    for number in (1..4).rev() {
        println!("{number}")
    }
    println!("LIFTOFF!!")
}