fn print_name(name : &String) {
    println!("{name}");
}

fn main() {
    let name = String::from("Lua");

    print_name(&name);

    println!("{name}"); // OK. bc we just borrowed "Lua" data to print_name with &.
}
