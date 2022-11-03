trait Duplicate {
    fn dub(&self) -> String;
}

impl Duplicate for String {
    fn dub(&self) -> String {
        format!("{0}-{0}", *self)
    }
}

impl Duplicate for i32 {
    fn dub(&self) -> String {
        format!("{}", *self * 2)
    }
}

fn duplication<T: Duplicate>(x: T) {
    println!("{}", x.dub())
}

pub fn stat() {
    let a = 42;
    let b = "Hi meysam".to_string();
    duplication(a);
    duplication(b);
}
