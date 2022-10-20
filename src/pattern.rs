pub fn pattern_type(amount: i32) -> &'static str {
    return match amount {
        0 => "double not found",
        1 | 2 => "a few double",
        3..=10 => "a lot double",
        _ if (amount % 2 == 0) => "even double",
        _ => "invalid",
    };
}

pub fn tuple_pattern(x: i32, y: i32) {
    let point = (x, y);
    match point {
        (0, 0) => println!("zero x,y"),
        (x, 0) => println!("x axis with {}", x),
        (0, y) => println!("y axis with {}", y),
        (x, y) => println!("x:{},y:{}", x, y),
    }
}
