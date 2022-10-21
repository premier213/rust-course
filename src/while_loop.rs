pub fn while_type(limit: u32) {
    let mut x = 1;
    while x * x < limit {
        println!("{0} * {0} = {1}", x, x * x);
        x += 1
    }
}

pub fn loop_standalone(limit: u32) {
    let mut x = 1;
    loop {
        println!("{0} * {0} * {0} = {1}", x, x * x * x);
        x += 1;
        if x * x * x > limit {
            break;
        }
    }
}
