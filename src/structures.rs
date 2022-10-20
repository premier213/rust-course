pub fn struct_type() {
    let employee = NewType {
        name: String::from("ali"),
        age: 2,
        company: String::from("google"),
    };

    println!("{}", employee.detail());
    println!("{}", NewType::static_detail())
}

#[derive(Debug)]
struct NewType {
    name: String,
    age: u32,
    company: String,
}

impl NewType {
    fn detail(&self) -> String {
        format!("{},{},{}", &self.name, &self.age, &self.company)
    }

    fn static_detail() -> String {
        String::from("data")
    }
}
