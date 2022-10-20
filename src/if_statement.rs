use rand::Rng;

pub fn statement() {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(1..10);
    if num > 5 {
        println!("higher than 5 , value is : {}", num)
    } else {
        println!("{}", num)
    }
}
