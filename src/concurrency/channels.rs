use std::{sync::mpsc, thread, time::Duration};

const NUM: usize = 20;

fn start(v: usize, tx: mpsc::Sender<usize>) {
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(v as u64));
        println!("sending : {}", v);
        tx.send(v).unwrap();
    });
}

pub fn channel() {
    // let (tx, rx) = mpsc::channel();
    // thread::spawn(move || tx.send(42).unwrap());
    // println!("{}", rx.recv().unwrap());

    let (tx, rx) = mpsc::channel();

    for i in 0..NUM {
        start(i, tx.clone());
    }

    for j in rx.iter().take(NUM) {
        println!("recv: {}", j);
    }
}
