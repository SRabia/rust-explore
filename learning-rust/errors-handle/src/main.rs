use std::{fs::File, panic};
use std::io::{ErrorKind, Write};

fn main() {
    println!("Hello, world!");
    // panic!("nooon");
    let g = File::open("hello.txt");
    let g_file = match g {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("problem creating file{:?}", e),
            },
            other_error => {
                panic!("other error {:?}", other_error);
            }
        },
    };


}
