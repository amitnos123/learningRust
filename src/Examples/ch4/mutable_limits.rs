fn main() {
    let mut s : String = String::from("hello");

    // Can have multiple immutable reference
    // Because the value doesn't change
    let r1: &String = &s;
    let r2: &String = &s;
    
    // Getting Error:
    // Cannot borrow `s` as mutable because it is also borrowed
    // as immutable mutable borrow occurs here.
    //
    // Immutable reference expect the value to be constant 
    // let r3: &String = &mut s;
    // println!("r1 = {}, r2 = {}, r3 = {}", r1, r2, r3);
    
    println!("r1 = {}, r2 = {}", r1, r2);

    // Here is out of r1 and r2 scope.
    // Then there is no immutable reference to s
    let r3: &String = &mut s;
    println!("r3 = {}", r3);
}
