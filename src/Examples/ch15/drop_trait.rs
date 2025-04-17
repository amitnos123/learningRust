struct CustomSmartPointer {
    data : String
}

// This let you implement a custom cleanup
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data)
    }
}

fn main() {
    let c = CustomSmartPointer{
        data : String::from("my stuff")
    };
    let d = CustomSmartPointer{
        data : String::from("other stuff")
    };
    // Output:
    // Dropping CustomSmartPointer with data `other stuff`!
    // Dropping CustomSmartPointer with data `my stuff`!

    // To drop manually
    drop(c); // This isn't the implemented drop function for Drop trait, but part of Rust library
}