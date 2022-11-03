use std::ops::Add;
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub fn over() {
    let p1 = Point { x: 3.4, y: 4.2 };
    let p2 = Point { x: 12.4, y: 3.2 };
    let p3 = p1 + p2;

    println!("axis {:?}", p3)
}
