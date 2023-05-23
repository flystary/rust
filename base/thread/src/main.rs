// use std::thread;
// use std::time::Duration;


// // 线程函数
// fn thread_fn() {
//     for i in 0..10 {
//         println!("hi number {} from the spawned thread!", i);
//         thread::sleep(Duration::from_secs(1));
//     }
// }

// fn main() {
//     // 创建子线程
//     let t = thread::spawn(thread_fn);

//     // 等待子线程结束
//     t.join();
// }



// fn main() {
//     let count = 5;

//     // 使用闭包创建线程
//     let t = thread::spawn(move || {
//         // 闭包捕获环境变量count
//         for i in 0..count {
//             println!("hi number {} from the spawned thread!", i);
//             thread::sleep(Duration::from_secs(1));
//         }
//     });
//     // 等待线程结束
//     t.join();
// }


// fn thread_fn(count: i32) {
//     for i in 1..count {
//         println!("hi number {} from the spawned thread!", i);
//         thread::sleep(Duration::from_secs(1));
//     }
// }

// fn main() {
//     let count = 5;
//     let  t = thread::spawn(move || {
//         thread_fn(count)
//     });

    
//     t.join();
// }

// use std::thread;
// use std::sync::mpsc;


// fn main()   {
//     let (tx, rx) = mpsc::channel();
//     let t = thread::spawn(move || {
//         let msg = rx.recv().unwrap();
//         println!("sub thread recv: {}", msg);
//     });
//     tx.send(String::from("hello")).unwrap();

//     t.join().unwrap();
// }


// use std::thread;
// use std::sync::mpsc;
// use std::time::Duration;

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     let t = thread::spawn(move || {
//         loop {
//             match rx.try_recv() {
//                 Ok(msg) => println!("sub thread recv: {}", msg),
//                 _ => {
//                     println!("sub thread recv nothing");
//                     thread::sleep(Duration::from_secs_f32(0.5))
//                 }
//             }
//         }
//     });

//     thread::sleep(Duration::from_secs_f32(3.5));

//     tx.send(String::from("hello")).unwrap();

//     t.join().unwrap();
// }


// use std::thread;
// use std::sync::mpsc;


// fn main() {
//     let (tx, rx) = mpsc::channel();
    

//     let tx1 = tx.clone();
//     let tx2 = tx.clone();

//     let t1 = thread::spawn(move || {
//         tx1.send(String::from("hello. I'm thread1.")).unwrap();
//     });

//     let t2 = thread::spawn(move || {
//         tx2.clone().send(String::from("hello. I'm thread2.")).unwrap();
//     });

//     let msg1 = rx.recv().unwrap();
//     println!("main thread recv: {}", msg1);

//     let msg2 = rx.recv().unwrap();
//     println!("main thread recv: {}", msg2);

//     t1.join().unwrap();
//     t2.join().unwrap();
// }


// use std::thread;
// use std::sync::mpsc;

// fn main() {
//     // 创建通道
//     let (tx, rx) = mpsc::channel();

//     let tx1 = tx.clone();
//     let tx2 = tx.clone();

//     // 启动子线程
//     let t1 = thread::spawn(move || {
//         tx1.send(String::from("hello. I'm thread1.")).unwrap();
//     });

//     let t2 = thread::spawn(move || {
//         tx2.clone().send(String::from("hello. I'm thread2.")).unwrap();
//     });

//     for msg in rx {
//         println!("main thread recv: {}", msg);
//     }
// }

// use std::thread;
// use std::sync::mpsc;

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     let t = thread::spawn(move || {
//         let msg = rx.recv().unwrap();
//         println!("sub thread recv: {}", msg);
//     });

//     let msg = String::from("hello");
//     tx.send(msg).unwrap();
//     // println!("main thread send: {}", msg);  //如果发送的消息被保存在某变量中的话，消息发送后，这个变量不能再被使用。


//     t.join().unwrap();
// }



// use std::sync::Mutex;

// fn main() {
//     let m = Mutex::new(vec![]);

//     let mut names = m.lock().unwrap();

//     names.push("main");
//     println!("{:?}", names);
// }


// use std::sync::Mutex;

// fn main() {
//     let m = Mutex::new(vec![]);

//     for i in 1..=10 {
//         let mut names = m.lock().unwrap();

//         names.push(format!("{}", i));
//     }

//     let names = m.lock().unwrap();
//     println!("{:?}", names);
// }


use std::thread;
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(vec![]);

    let mut threads = vec![];
    for i in 1..10 {
        let t = thread::spawn(move || {
            let mut names = m.lock().unwrap();
            names.push(format!("{}", i));
        });

        thread::push(t);
    }

    let names = m.lock().unwrap();

    println!("{:?}", names);
}









// use std::thread;
// use std::sync::{Mutex, Arc};

// fn main() {
//     // Arc，它的内部为之前的信号量
//     let m = Arc::new(Mutex::new(vec![]));
    
//     // 循环中创建多个线程
//     let mut threads = vec![];
//     for i in 1..10 {
//         // m具有了clone方法
//         let m = m.clone();

//         // 创建线程
//         let t = thread::spawn(move ||{
//             // Arc类型可以直接使用内部的值，从信号量中取得共享内存的方法与不使用Arc完全一致
//             let mut names = m.lock().unwrap();

//             // 修改共享内存
//             names.push(format!("thread{}", i));
//         });
        
//         threads.push(t);
//     }

//     // 等待所有线程结束
//     for t in threads {
//         t.join().unwrap();
//     }
    
//     // 打印接龙名单
//     let names = m.lock().unwrap();
//     println!("{:?}", names);
// }
