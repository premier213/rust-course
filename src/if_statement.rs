use rand::Rng;

pub fn statement() {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(1..10);
    if num > 5 {
        println!("higher than 5 , value is : {}", num)
    } else if num == 5 {
        println!("equal to 5 ,value is :{}", num)
    } else {
        println!("your number is {}", num)
    }

    let num2 = rng.gen_range(1..5);
    let value = if num2 > 2 { true } else { false };
    println!("number higher than 2 , {},{}", value, num2)
}
