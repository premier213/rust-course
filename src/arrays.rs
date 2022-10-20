pub fn array_type() {
    let primes = [1, 2, 3, 4];
    let double: [f64; 3] = [2.3, 3.4, 1.1];
    println!("{:?}", primes);
    println!("{:?}", double);

    let mut number = [1; 9];
    number[3] = 5;
    println!("{:?}", number);

    for num in number.iter() {
        println!("{}", num)
    }
}
