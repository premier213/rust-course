trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut sum = 0;
        for i in self {
            sum += *i;
        }
        sum
    }
}

pub fn mut_sum() {
    let a = vec![1, 2, 3, 4];
    println!("sum= {}", a.sum());
    // let b = vec![4.0, 4.0, 4.1, 4.5];
    // println!("sum b = {}", b.sum())
}
