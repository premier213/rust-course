pub fn closures_type() {
    let a = |a: i32| a + 1;
    println!("{}", a(4));

    let b = |b: i32| -> i32 {
        let c = b + 2;
        c
    };

    println!("{}", b(4));

    let gen = |x| println!("{}", x);
    gen(4);
}
