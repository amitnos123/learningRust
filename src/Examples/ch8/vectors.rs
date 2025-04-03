fn main() {
    let mut v : Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let v2 : Vec<i32> = vec![1, 2, 3];


    let third : &i32 = &v[2];
    println!("The third element is {}", third);

    // If we aren't sure, if v[2] has some value.
    // To prevent error during run in case of no value at given index.
    match v.get(20) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element.")
    };


    // Iterate over the vector
    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }


    // Enum + Vectors
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    // Vector with diffierent types
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];

    match &row[1] {
        SpreadsheetCell::Int(i) => println!("{}", 1),
        _ => println!("Not a interger!")
    };
}