/*
        THREADS
*/

// use std::sync::mpsc;
// use std::thread;

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move || {
//         let val = String::from("hi");
//         tx.send(val).unwrap();
//     });

//     // we can use `recv()` or `try_recv()` to get the message
//     // we use `try_recv()` when we want the program to try to receive a message 
//     // and if there is no message continue doing something, than later try again
//     let received = rx.recv().unwrap();
//     println!("Got: {}", received);
// }

// ** Sending multiple values and seeing the receiver waiting

// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread")
//         ];

//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     for received in rx {
//         println!("Got: {}", received);
//     }
// }

// ** Creating multiple producers by cloning the transmitter

// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     let tx1 = tx.clone();
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread")
//         ];

//         for val in vals {
//             tx1.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     thread::spawn(move || {
//         let vals = vec![
//             String::from("more"),
//             String::from("messages"),
//             String::from("for"),
//             String::from("you")
//         ];

//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     for received in rx {
//         println!("Got: {}", received);
//     }
// }

/*
        MUTEX
*/

// use std::sync::Mutex;

// fn main() {
//     let m = Mutex::new(5); // type Mutex<i32>

//     {
//         // in order to access m we must call `lock()` on it first
//         let mut num = m.lock().unwrap(); // treat num as a mutable reference returned from the mutex
//         *num = 6;
//     } // lock is released here automatically

//     println!("m = {:?}", m);
// }

// ** Sharing a Mutex between multiple threads

use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
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