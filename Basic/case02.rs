struct Player {
    hp: i32,
    name: String,
}

impl Player {

    fn new(hp: i32, name: String) -> Player {
        Player { hp, name }
    }

    fn attack(&self) {
        println!("{} attacks.", self.name);
    }
}

fn main() {
    let player1 = Player::new(100, String::from("Player1"));

    player1.attack();
    println!("{}'s HP is {}.", player1.name, player1.hp);
}

// Rust is funny as Lua!