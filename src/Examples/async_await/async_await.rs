use tokio::time::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let f = my_function(-1);
    println!("Let's Get Rusty!");
    f.await;

    let mut handles = vec![];

    for i in 0..2 {
        let handle = tokio::spawn(async move {
            my_function(i).await;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

async fn my_function(i : i32) {
    // futures are lazy
    // mean they won't do nothing until they are polled
    println!("[{}] I'm an sync function!", i);
    let s1= read_from_database().await;
    println!("[{}] First Result: {}", i, s1);
    let s2 = read_from_database().await;
    println!("[{}] First Result: {}", i, s2);
}

async fn read_from_database() -> String {
    sleep(Duration::from_millis(50)).await;
    "DB Result".to_owned()
}