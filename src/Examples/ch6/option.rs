fn main() {
    /* Part 1 of Code */

    let x : i8 = 5;
    let y : Option<i8> = Some(5);

    // If y has some value, use it
    // Else (y is None) use default value, 0
    let sum : i8 = x + y.unwrap_or(0);

    /* Part 2 of Code */

    let five = Some(5);
    let size = plus_one(five);
    let none = plus_one(None);

    /* Part 3 of Code */

    let some_value = Some(3);
    if let Some(3) = some_value {
        println!("three")
    }
}

fn plus_one(x : Option<i32>) -> Option<i32> {
    // Can write like this
    // match x {
    //     None => None,
    //     Some(i) => Some(i + 1)
    // }

    // Another equal way:
    match x {
        Some(i) => Some(i + 1),
        _ => None // _ is default value
    }
}