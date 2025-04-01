fn main() {
    let mut s : String = String::from("hello world");
    
    // let hello : &str = &s[0..5]; // String slice
    let hello : &str = &s[..5]; // String slice
    // Can remove the 0, because it's start of the String

    // let world : &str = &s[6..11]; // String slice
    let world : &str = &s[6..]; // String slice
    // Can remove the 11, because it's end of the String

    let hello_world : &str = &s[..]; // return a slice String for the whole String

    let word : &str = first_word(&s);

    println!("the first word is: {}", word);
}


// &String automatecly converted to &str
fn first_word( s: &str) -> &str {
    let byte : &[u8] = s.as_bytes();

    for (i, &item) in byte.iter().enumerate() {
        if item == b' ' { // check if item is space in binery
            return &s[..i]
        }
    }

    &s[..]
}