use Level::{Easy, Hard, Middle};

pub fn matching(level: Level) {
    match level {
        Hard => {
            println!("this level is hard")
        }
        Middle => {
            println!("this level is middle")
        }
        Easy => {
            println!("this level is easy")
        }
    }
}

pub fn matching_name(name: String) {
    match name.as_str() {
        "ali" => println!("my name is ali2"),
        "mohammad" => println!("my name is mohammad2"),
        _ => println!("my name is {}", name),
    }
}

pub enum Level {
    Hard,
    Middle,
    Easy,
}
