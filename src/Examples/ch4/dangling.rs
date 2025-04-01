fn main() {
    let reference_to_nothing = dangle();
}

// Return reference to s.
// s is out of scope when dangle function end
// Hence refering to nothing
//
// Rust compiler prevent this
fn dangle() -> &String {
    let s : String = String::from("hello");

    &s
}