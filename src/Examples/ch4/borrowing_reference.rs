fn main() {
    let s1 : String = String::from("Hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);

    let mut s1 : String = String::from("hello");
    change(&s1);
    println!("s1 = {}", s1);
}

// & character indicates sending and reciving a reference
// refence are mutable by default
// This is borrowing the value from s1, but not taking ownership of s1
fn calculate_length(s : &String) -> usize {
    let length : usize = s.len();
    length
}

// &mut is taking mutable reference
// You can only have a single mutable reference in a scope.
// This prevent data races on the variable
fn change(some_string: &mut String) {
    some_string.push_str("m would");
}