pub fn print_line() {
    //! print line
    println!("Hello, world!");
    println!("My name is {} and I'm {} years old", "Alex", 29);
    println!("a + b = {}", 3 + 9);
    println!("{0} has a {2} and {0} has a {1}", "Alex", "cat", "dog");
    println!("{name} {surname}", surname = "Smith", name = "Alex");
    println!("binary: {:b}, hex: {:x}, octal: {:o}", 50, 50, 50);
    println!("array: {:?}", [1, 2, 3]);
}
pub fn strings() {
    //! strings
    let cat: &'static str = "fluffy";
    let dog: &str = "jess";
    let mut name = String::from("my name is meysam"); //string object
    println!("{}", name.len()); //get length of string
    name.push(' '); // push character end of string
    name.push_str(",i have 30 years old"); //push to string
    println!("{}", name);
    let new_name = name.replace("meysam", "ali"); // replace part of string
    let split = name.split(','); //split string
    let whitespace = name.split_whitespace(); //split whitespace
    println!("{}", new_name);
    println!("{:#?}", split);
    println!("{:#?}", whitespace);
}

pub fn format() {
    //! format print
    let name = String::from("hello owner");
    let owner = format!("hi ,{name}");
    println!("{owner}")
}
