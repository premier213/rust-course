use std::{
    fs::File,
    io::{self, Read},
};

fn read() -> Result<String, io::Error> {
    let mut f = File::open("src/username.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

pub fn operator() {
    let a = read();

    println!("{:?}", a)
}
