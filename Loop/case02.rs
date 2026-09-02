fn main() {
    let mut x = 2;

    // 'outer is label. there is a label system in rust :O
    'outer: loop {
        let mut y = 0;

        loop {
            println!("x={}, y={}", x, y);
            y += 1;

            if y == 2 && x == 2 {
                break 'outer;
            }

            y += 1;

            if y == 5 {
                break;
            }
        }

        x += 1;
    }
}