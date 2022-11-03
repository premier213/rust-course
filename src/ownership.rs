pub fn owner() {
    let v = vec![1, 2, 3, 4, 5];

    let foo = |v: Vec<i32>| -> Vec<i32> {
        println!("foo");
        v
    };

    let v = foo(v);

    println!("{:?}", v)
}
