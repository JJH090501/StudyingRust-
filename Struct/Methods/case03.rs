struct Car {
    speed: u32,
    power: u32,
}

impl Car {
    fn can_hold(&self, other: &Car) -> bool {
        self.speed > other.speed && self.power > other.power
    }
}

fn main() {
    let car1 = Car {
        speed: 50,
        power: 30,
    };

    let car2 = Car {
        speed: 25,
        power: 10,
    };

    println!("{}", car1.can_hold(&car2));
}