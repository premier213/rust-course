use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn random() {
    let mut ringo = rand::thread_rng();
    let i: i32 = ringo.gen();
    println!("{i}");
    println!("integer: {}", ringo.gen_range(1..=10));
    println!("float: {}", ringo.gen_range(1.1..=10.10));

    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .map(char::from)
        .take(30)
        .collect();

    println!("random alphanumeric {}", rand_string);
}
