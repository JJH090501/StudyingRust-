fn main() {
    struct Case1 {}
    struct Case2 {}

    enum Cases {
        C1(Case1),
        C2(Case2),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

}