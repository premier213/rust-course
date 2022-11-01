macro_rules! first_macro {
    () => {
        println!("first macro");
    };
}

macro_rules! hello_macro {
    ($name:expr) => {
        println!("hello, {}", $name)
    };
}

macro_rules! multi_value {
    ($($name:expr),*) => ($(println!("multi value : {}", $name);)*)
}

macro_rules! catches {
    (x=>$e:expr) => {
        println!("x value is: {}", $e)
    };
    (y=>$e:expr) => {
        println!("y value is: {}", $e);
    };
}

pub fn macro_type() {
    first_macro!();
    hello_macro!("ali");
    multi_value!("mohammad", "meysam", "ali");
    catches!(x=>5);
    catches!(y=>1);
}
