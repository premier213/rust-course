static mut R: i32 = 0;

pub fn unsafe_type() {
    unsafe {
        R = 4;
        println!("in scope 1 = {}", R);
    }
    unsafe {
        println!("in scope 2 = {}", R);
    }
}
