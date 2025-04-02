enum IpAddr {
    V4(u8,u8,u8,u8),
    V6(String)
}

enum Message {
    Quit,
    Move { x : i32, y : i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn some_function() {
        println!("Let's Get Rusty!")
    }
}

fn main() {
    let localhost : IpAddr = IpAddr::V4(127, 0, 0, 1);
}