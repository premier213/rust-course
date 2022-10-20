pub fn tuple_type() {
    let mut person = (2, 3, "hello");
    person.0 = 10;
    println!("{}", person.0);

    let (name, age, hi) = person;

    println!("{}{}{}", name, age, hi)
}
