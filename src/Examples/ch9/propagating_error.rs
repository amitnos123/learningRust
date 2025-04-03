use std::fs::{self, File};
use std::io::{self, Read};

fn main() {
    // unwrap() is good to use when you don't need to handle errors.
    // For example during test, or parsing a static String
}

// Both option A and option B are equal

fn read_username_from_file_option_a() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file, // Return file on openning success
        Err(e) => return Err(e) // Return error on failing to open
    };

    let mut s = String::new();

    // read_to_string return Result type
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

// Adding question mark to end of the line
// Is Equal to:
// let mut f = match f {
//     Ok(file) => file,
//     Err(e) => return Err(e)
// };

fn read_username_from_file_option_b() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    
    // Equal Code 
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    
    Ok(s)
}

fn read_username_from_file_option_c() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}