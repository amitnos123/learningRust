fn main() {
    // Strings are stored as a collection of UTF-8 encoded bytes
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');

    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 : String = s1 + &s2; // We are moving ownership from s1 to s3 and pending a slice of s2 to it 


    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = format!("{}{}", s1, s2); // Another way to connect strings


    let hello : String = String::from("Hello");
    // Will get an error
    // let c : char = hello[0];
    // hello[0] call first byte, not char
}