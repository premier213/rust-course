use std::fs::File;

pub fn err() {
    // panic!("wrong panic code");

    let f = File::open("main.jpg");

    match f {
        Ok(f) => {
            println!("file found {:?}", f);
        }
        Err(e) => {
            println!("error : \n{:#?}", e)
        }
    }

    println!("excute")
}
