pub fn enum_type() {
    let my_color = Colors::Blue;
    println!("{:?}", my_color);

    let person = Person::Name(String::from("ali"));
    println!("{:?}", person)
}

#[allow(dead_code)]
#[derive(Debug)]
enum Colors {
    Red,
    Blue,
    Green,
}
#[allow(dead_code)]
#[derive(Debug)]
enum Person {
    Name(String),
    Surname(String),
    Age(u32),
}
