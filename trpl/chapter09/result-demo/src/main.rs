use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating file: {:?}", e)
            }
            oe => panic!("Error opening file {:?}", oe),
        },
    };

    let f2 = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Error creating file: {:?}", error);
            })
        } else {
            panic!("Error opening file: {:?}", error);
        }
    });
}
