pub fn borrow() {
    //! borrow function
    let mut name = "ali";
    let ret = borrowing(&mut name);
    println!("{}", ret);
}

pub fn borrowing(value: &mut &str) -> String {
    //! borrow string
    let val = format!("hello {}", value);

    val //return value
}
