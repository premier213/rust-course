pub fn generic_types() {
    let p1: Point<i32> = Point { x: 6, y: 8 };
    let p2: Point<f64> = Point { x: 1.1, y: 4.3 };
    println!("{:?},{:?},{:?},{:?}", p1.x, p2.x, p1.y, p2.y);

    let c1 = Colors::Blue("#eee");
    let c2 = Colors::Red(255);
    println!("{:?},{:?}", c1, c2);

    let p3: Point2<u64, i32> = Point2 { x: 4, y: 3 };
    println!("{:?},{:?}", p3.x, p3.y);
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
#[derive(Debug)]
enum Colors<T> {
    Red(T),
    Blue(T),
}

#[derive(Debug)]
struct Point2<T, V> {
    x: T,
    y: V,
}
