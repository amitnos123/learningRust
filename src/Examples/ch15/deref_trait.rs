use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Deref trait require implementation of function deref
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0 // Return first element in tuple struct
    }
}
// DerefMut is same just for mutable

fn main() {
    let x = 5;
    let y = &x;
    let z = Box::new(x);
    let w = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // assert_eq!(5, y); // Will return error
    assert_eq!(5, *z);
    assert_eq!(5, *w); // Allowed because implemented Deref trait

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // Rust implement by itself &MyBox<String> -> &String -> &str
}

fn hello(name : &str) {
    println!("Hello,, {}!", name)
}