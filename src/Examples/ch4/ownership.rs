fn main() {
    let s1 : String = give_ownership();
    let s2 : String = String::from("world");
    let s3 : String = take_gives_back_ownership(s2);

    println!("s2 = {}", s2); // Will return an error
    println!("s1 = {}, s3 = {}", s1, s3);
}

fn give_ownership() -> String {
    // Generate a String variable
    let some_string : String = String::from("hello");

    // return the variable and gives ownership over it.
    some_string
}

// The function take ownership of variable a_string
fn take_gives_back_ownership(a_string : String) -> String {
    // return the variable and gives ownership over it.
    a_string
}