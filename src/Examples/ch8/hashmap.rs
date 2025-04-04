use std::collections::HashMap;

fn main() {
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 20);

    let team_name = String::from("Blue");
    let score : Option<&i32> = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }



    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 20);

    scores.entry(String::from("Yellow")).or_insert(30); // If Yellow doesn't exist, insert 30 in Yellow key
    scores.entry(String::from("Yellow")).or_insert(40); // If Yellow doesn't exist, insert 40 in Yellow key



    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}