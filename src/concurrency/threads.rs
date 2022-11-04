use std::{thread, time::Duration};

pub fn thread() {
    let mut thread = vec![];
    for i in 1..10 {
        let th = thread::spawn(move || {
            thread::sleep(Duration::from_millis(i * 1000));
            println!("in thread : {}", i)
        });

        thread.push(th);
    }

    for i in 1..10 {
        println!("out of thread: {}", i)
    }
    for i in thread {
        i.join().expect("err");
    }

    println!("main");
}
