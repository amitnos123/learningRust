use std::fs::File;

fn main() {
    // On success will return file, else panic
    // let f = File::open("hello.txt").unwrap();
    // Expect return the message on error as panic
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}