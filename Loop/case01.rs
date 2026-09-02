fn main() {

    let mut cnt = 2; // mut is constant variable. You can change it.

    let result = loop {
        print!("Hello, Lua!")
        println!(" I am Rust!")

        cnt += 1;

        if cnt == 3 {
            continue;
        }

        println!("cnt={}", cnt);

        if cnt == 5 {
            break cnt; // break can return value.
        }

    };

    println!("result={}", result);
}

// this is real infinite loop. Xpp u can stop it with break;