fn main() {
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    } // Here result lifetime end, because string2 lifetime end. longest reference return has the lifetime of string2 (because it's shorter then string1 lifetime)
}

// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

// lifetime always written in <> brakets
// lifetime always start with '
// This tell the compiler that the returned reference has the shortest lifespan of x and y 
fn longest<'a>(x : &'a str, y : &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}