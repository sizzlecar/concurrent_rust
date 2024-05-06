use std::{
    sync::Arc,
    thread::{self, JoinHandle},
};

///rust中并发相关的基础知识
/// 1. 如何创建线程
fn main() {
    //rust创建完线程之后直接启动，与java不同
    create_thread();
    println!("main thread Hello, world!");
    //这里主线程如果不等一下，会直接退出，导致子线程也退出
    thread::sleep(std::time::Duration::from_secs(1));

    //返回JoinHandle，可以等待线程结束
    let handle = create_thread_2();
    handle.join().unwrap();
    println!("main thread wait handle");

    //创建线程并返回值
    let handle = create_thread_with_return();
    let res = handle.join().unwrap();
    println!("main thread wait handle return value: {}", res);

    //线程中使用move
    let handle = create_thread_with_move();
    handle.join().unwrap();
}

fn create_thread() {
    thread::spawn(|| {
        println!("new thread Hello, world!");
    });
}

fn create_thread_2() -> JoinHandle<()> {
    thread::spawn(|| {
        println!("new thread return handle!");
    })
}

fn create_thread_with_return() -> JoinHandle<String> {
    thread::spawn(|| {
        println!("new thread return handle!");
        String::from("return value")
    })
}

fn create_thread_with_move() -> JoinHandle<()> {
    let arc = Arc::new(1);
    let arc_1 = arc.clone();
    let jh = thread::spawn(move || {
        println!("new thread val : {}", *arc_1); // 在新线程中使用 `arc_clone`
    });
    println!("main thread val : {:?}", *arc); // 在主线程中使用 `arc`
    jh
}
