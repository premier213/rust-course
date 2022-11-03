#[derive(Debug)]
struct Person {
    name: String,
}
#[derive(Debug)]
struct Dog<'l> {
    name: String,
    owner: &'l Person,
}

impl Person {
    fn gen_name<'l>(&'l self) -> &'l String {
        &self.name
    }
}

fn get_str() -> &'static str {
    "hello"
}

pub fn life() {
    println!("{}", get_str());

    let p1 = Person {
        name: String::from("json"),
    };
    let d1 = Dog {
        name: String::from("dog"),
        owner: &p1,
    };

    println!("{:?}", d1);

    let mut a: &String;
    {
        let p2 = Person {
            name: String::from("mary"),
        };
        a = p1.gen_name();
    }

    println!("{}", a)
}
