const ANSWER: u32 = 10;
pub fn err(x: Option<u32>) {
    match x {
        Some(0) => panic!("panicked"),
        Some(x) => println!("result is {}", ANSWER / x),
        None => println!("none is {}", ANSWER),
    }
}
