use std::{
    fs::{remove_file, File, OpenOptions},
    io::{Read, Write},
};

pub fn files() {
    // create and write file
    let mut file = File::create("./src/example.txt").expect("msg");
    file.write_all("hello newfile".as_bytes()).expect("error");

    //append file
    let mut file_write = OpenOptions::new()
        .append(true)
        .open("./src/example.txt")
        .expect("cannot");
    file_write
        .write_all("\nworld!!".as_bytes())
        .expect("can not write");

    // read file
    let mut read = File::open("src/example.txt").unwrap();
    let mut contents = String::new();
    read.read_to_string(&mut contents).unwrap();

    //delete file
    remove_file("src/example.txt").expect("delete problem");

    println!("{}", contents);
}
