use std::{fs::File, io};
use std::io::Read;

fn main() {
    match open_file_and_read("hello.txt") {
        Ok(s) => println!("The file content is {}", s),
        Err(_) => println!("Failed to read the file"),
    }

    //This application will alwas panic because of the next function call
    always_fails();
}

fn always_fails() {
    panic!("This will always crash the running application.") //This is called an unrecoverable error. You cannot do anything against the application crash
}

fn open_file_and_read(filename: &str) -> Result<String, io::Error> {
    let mut f = File::open(filename)?; //The ? at the end of this line will propagate the error case. This way the user of this function has to handle it and not us
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
