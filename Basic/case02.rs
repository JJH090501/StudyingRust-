struct Player {
    hp: i32,
    name: String,
}

impl Player {
    fn attack(&self) {
        println!("{} attacks.", self.name);
    }
}

fn main() {
    let player1 = Player {
        hp: 100,
        name: String::from("LuLu"),
    };

    player1.attack();
    println!("{}'s HP is {}.", player1.name, player1.hp);
}

// Rust is funny as Lua!