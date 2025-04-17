use std::rc::Rc;
use std::cell::RefCell;

enum ListBox {
    ConsBox(i32, Box<ListBox>),
    NilBox
}

enum ListRc {
    ConsRc(i32, Rc<ListRc>),
    NilRc
}

use ListBox::{ConsBox, NilBox};
use ListRc::{ConsRc, NilRc};

fn main() {
    // Box<T> for allocating values on the heap
    // Rc<T>, a reference counting type that enables multiple ownership
    // Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time
    let A = ConsBox(5, Box::new(ConsBox(10, Box::new(NilBox))));
    let B = ConsBox(3, Box::new(A));
    let C = ConsBox(4, Box::new(A));

    let a = Rc::new(ConsRc(5, Rc::new(ConsRc(10, Rc::new(NilRc)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = ConsRc(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = ConsRc(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    let value = Rc::new(RefCell::new(5));

    let d = Rc::new(ConsRc(Rc::clone(&value), Rc::new(NilRc)));
    let e = Rc::new(ConsRc(Rc::clone(3), Rc::new(&a)));
    let f = Rc::new(ConsRc(Rc::clone(4), Rc::new(&a)));

    println!("d before = {:?}", d);
    println!("e before = {:?}", e);
    println!("f before = {:?}", f);

    *value.borrow_mut() += 10;

    println!("d after = {:?}", d);
    println!("e after = {:?}", e);
    println!("f after = {:?}", f);
}