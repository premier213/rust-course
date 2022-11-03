use std::rc::Rc;

struct Car {
    brand: Rc<String>,
}

impl Car {
    fn new(brand: Rc<String>) -> Car {
        Car { brand: brand }
    }

    fn drive(&self) {
        println!("{} is car brand", &self.brand)
    }
}

pub fn reference() {
    let brand = Rc::new(String::from("bmw"));
    println!("p :{}", Rc::strong_count(&brand));
    {
        let car = Car::new(brand.clone());
        car.drive();
        println!("p: {}", Rc::strong_count(&brand));
    }

    println!("my car is {}", brand);
    println!("my p {}", Rc::strong_count(&brand));
}
