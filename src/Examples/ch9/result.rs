use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");  

    // Option A to handle error
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error)
    // };

    // Option B to handle error
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e)
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        }
    };
}