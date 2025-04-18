use std::{thread, time::Duration};
fn main() {
    // Then the main thread ends, the spawn threads also ends.
    // This doesn't matter about the state of the spawn thread
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the spawned thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Will block the currently running thread at this point,
    // until the thread associated with the handle terminates
    handle.join().unwrap();

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || { // Telling the thread to take ownership of v
        println!("Here's a vector: {:?}", v)
    });
}