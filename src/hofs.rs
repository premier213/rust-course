pub fn hof_type(f: fn(i32) -> i32, a: i32) {
    println!("hof : {}", f(a))
}
