fn dangle() -> &String {
    let s = String::from("hello");

    &s // can't return local var's reference value
}