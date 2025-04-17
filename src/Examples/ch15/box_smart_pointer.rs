enum List {
    Cons(i32, Box<List>),
    Nil
}

use List::{Cons, Nil};

fn main() {
    // Smart pointers, on the other hand,
    // are data structures that act like a pointer
    // but also have additional metadata and capabilities. 

    // Box<T> for allocating values on the heap
    // Rc<T>, a reference counting type that enables multiple ownership
    // Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time
    
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}