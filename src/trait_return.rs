struct Dog {}

struct Cat {}
trait Color {
    fn check_color(&self) -> &'static str;
}

impl Color for Dog {
    fn check_color(&self) -> &'static str {
        return "black dog";
    }
}

impl Color for Cat {
    fn check_color(&self) -> &'static str {
        return "white cat";
    }
}

fn get_color(v: f64) -> Box<dyn Color> {
    if v < 1.0 {
        Box::new(Dog {})
    } else {
        Box::new(Cat {})
    }
}

pub fn trait_ret() {
    println!("has {} color", get_color(1.2).check_color())
}
