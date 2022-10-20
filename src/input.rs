use std::io;

pub fn input() {
    //! user input
    let mut input = String::new();

    println!("{}", input);

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("you said {}", input);
        }
        Err(e) => {
            println!("somthing {}", e);
        }
    };
}
