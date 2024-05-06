use std::{
    sync::{Arc, Mutex, RwLock},
    thread,
};

///rust中多线程共享内存
fn main() {
    share_memory_arc();
    share_memory_mutex();
    share_memory_read_write_lock();
}

fn share_memory_arc() {
    //Arc是一个原子引用计数类型，可以安全的在线程间共享
    let arc = Arc::new(999);
    let mut handles = vec![];
    for _ in 0..10 {
        let arc_clo = Arc::clone(&arc);
        let handle = thread::spawn(move || {
            println!(
                "thread id: {:?} arc value: {}",
                thread::current().id(),
                *arc_clo
            );
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn share_memory_mutex() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    let mut handles = vec![];

    for i in 0..10 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data.push(i);
            println!(
                "thread id: {:?} data value: {:?}",
                thread::current().id(),
                *data
            );
            //锁会在data离开作用域时自动释放
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", *data.lock().unwrap());
}

fn share_memory_read_write_lock() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    let mut handles = vec![];

    for i in 0..10 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            if i % 2 == 0 {
                let data = data.read().unwrap();
                println!(
                    "thread id: {:?} read data value: {:?}",
                    thread::current().id(),
                    *data
                );
            } else {
                let mut data = data.write().unwrap();
                data.push(i);
                println!(
                    "thread id: {:?} write data value: {:?}",
                    thread::current().id(),
                    *data
                );
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", *data.read().unwrap());
}
