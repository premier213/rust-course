pub fn vector_type() {
    let vacs: Vec<i32> = Vec::new();
    println!("{:?}", vacs);

    let mut vacs2: Vec<u32> = vec![2, 3, 4];
    println!("{:?}", vacs2);

    vacs2.push(7);
    println!("{:?}", vacs2);

    vacs2.remove(2);
    println!("{:?}", vacs2);

    let vacs3 = vec![2; 5];
    println!("{:?}", vacs3);

    for vac in vacs3.iter() {
        println!("{}", vac)
    }
}
