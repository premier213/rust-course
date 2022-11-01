struct Dog {
    color: &'static str,
}

struct Cat {
    gen: u32,
}

trait Special {
    fn special_generation(&self) -> String;
}

impl Special for Dog {
    fn special_generation(&self) -> String {
        return format!("{}", self.color);
    }
}

impl Special for Cat {
    fn special_generation(&self) -> String {
        return format!("{}", self.gen);
    }
}

fn generate<T: Special>(v: T) {
    println!("{} sp", v.special_generation())
}

pub fn gn() {
    let dog = Dog { color: "white" };
    let cat = Cat { gen: 10 };
    generate(dog);
    generate(cat);
}
