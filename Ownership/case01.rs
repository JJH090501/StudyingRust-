fn print_name(name : String) {
    println!("{name}");
}

fn main() {
    let name = String::from("Lua");

    print_name(name);

    println!("{name}"); // it is ERROR.  so we need .clone() bc now the ownership of "Lua" is print_name(), not name
}