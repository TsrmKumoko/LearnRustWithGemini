use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Mutex, Arc};

pub fn threads_example() {
    // 线程（Threads）
    // Rust 提供了轻量级的绿色线程（green threads）实现，但现在标准库使用的是操作系统线程。
    // 线程允许程序的不同部分并发运行。

    // 创建新线程
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 等待所有子线程完成
    handle.join().unwrap();

    // `move` 闭包与线程
    // `move` 关键字强制闭包获取其捕获变量的所有权，这对于将值从一个线程转移到另一个线程非常有用。
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector from the spawned thread: {:?}", v);
    });

    handle.join().unwrap();

    // 消息传递（Message Passing）
    // Rust 的消息传递并发模型允许线程通过发送消息来相互通信，而不是共享内存。
    // 使用 `mpsc`（multiple producer, single consumer）通道。
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    // 共享状态并发（Shared-State Concurrency）
    // 多个线程可以访问同一块数据。Rust 通过 `Mutex` 和 `Arc` 来保证共享数据的安全访问。

    // Mutex（互斥锁）
    // `Mutex` 允许一次只有一个线程访问数据。
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}