use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone();

    thread::spawn(move || {
        let msg = String::from("hi");
        tx.send(msg).unwrap();
        // println!("msg is {}", msg); // thread doesn't own msg anymore

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread")
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1))
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you")
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1))
        }
    });

    // Will wait for value
    // If channel closes, then return error
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // Will not wait for the value
    // If didn't receive a value yet, return error
    // let received = rx.try_recv().unwrap();
    // println!("Got: {}", received);
}