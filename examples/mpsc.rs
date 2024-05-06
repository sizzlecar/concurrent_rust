use std::sync::mpsc;
use std::thread;

/// rust mpsc example
fn main() {
    simple_mpsc();

    mpsc_with_multiple_producers();

    mpsc_with_rx_iter();
}

fn simple_mpsc() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    // recv() 方法从通道中接收值, 如果值还没有准备好, recv 会阻塞当前线程
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn mpsc_with_multiple_producers() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let val = String::from("foo");
        tx.send(val).unwrap();
    });

    thread::spawn(move || {
        let val = String::from("bar");
        tx1.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn mpsc_with_rx_iter() {
    let (tx, rx) = mpsc::channel();
    for i in 1..10 {
        let tx1 = tx.clone();
        thread::spawn(move || {
            let val: String = i.to_string();
            tx1.send(val).unwrap();
        });
    }

    for received in rx.try_iter() {
        println!("Got: {}", received);
    }
}
