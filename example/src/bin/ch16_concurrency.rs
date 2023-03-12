use std::sync::mpsc; // multi producer single consumer
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    // thread
    {
        // thread 使用 spawn 创建新线程
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

        // 使用 join 等待所有线程结束
        handle.join().unwrap();
    }

    {
        // move
        let v = vec![1, 2, 3];

        //move 关键字强制闭包获取其使用的环境值的所有权。创建新线程将值的所有权从一个线程移动到另一个线程。
        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });

        handle.join().unwrap();
    }

    // message passing 使用消息传递在线程间传送数据
    {
        // tx: transmitter, rx: receiver
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });

        let received = rx.recv().unwrap();
        println!("Got1: {}", received);
    }

    {
        // multi-message
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            // 发送多个消息，并在每次发送后暂停一段时间
            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got2: {}", received);
        }
    }

    {
        // muiti-producer 通过克隆发送者来创建多个生产者
        let (tx, rx) = mpsc::channel();

        let tx1 = tx.clone();
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
            println!("Got3: {}", received);
        }
    }

    // 互斥器mutex，即mutual exclusion
    {
        let m = Mutex::new(5);
        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }
        println!("m = {:?}", m);
    }

    {
        // mutex in multi-thread
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            // 使用 Arc<T> 包装一个 Mutex<T> 能够实现在多线程之间共享所有权。原子引用计数（atomically reference counted）
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
}

// std::marker 中的 Sync 和 Send trait。

// 通过 Send 允许在线程间转移所有权。
//Send 标记 trait 表明实现了 Send 的类型值的所有权可以在线程间传送。几乎所有的 Rust 类型都是Send 的；不过有一些例外，包括 Rc<T>。
// 几乎所有基本类型都是 Send 的。任何完全由 Send 的类型组成的类型也会自动被标记为 Send。

// Sync 允许多线程访问。
// Sync 标记 trait 表明一个实现了 Sync 的类型可以安全的在多个线程中拥有其值的引用。
// 基本类型是 Sync 的，完全由 Sync 的类型组成的类型也是 Sync 的。
