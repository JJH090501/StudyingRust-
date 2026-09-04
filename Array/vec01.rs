fn main() {
    let mut numbers: Vec<i32> = Vec::new();

    numbers.push(10);
    numbers.push(20);
    numbers.push(30);

    println!("numbers = {:?}", numbers)
}

// {}	일반 출력 (Display)	10
// {:?}	디버그 출력 (Debug)	[1, 2, 3]
// {:#?}	예쁘게 펼친 Debug 출력	struct, Vec 등
// {:b}	2진수	1010
// {:x}	16진수 소문자	ff
// {:X}