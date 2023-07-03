// imports the necesssary crates
use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn to_ctrl_byte(c: char) -> u8 {
    // converts the char to a byte
    let byte = c as u8;
    // returns the byte
    byte & 0b0001_1111
}

fn die(e: std::io::Error) {
    // prints the error
    panic!("{}", e);
}

// main function
fn main() {
    // creates a raw terminal
    let _stdout = stdout().into_raw_mode().unwrap();
    
    // reads the bytes from stdin
    for b in io::stdin().bytes() {
        // matches the byte
        match b {
            // if the byte is Ok
            Ok(b) => {
                // converts the byte to a char
                let c = b as char;
                
                // checks if the char is a control char
                if c.is_control() {
                    println!("{:?}\r", b);
                } else {
                    // difference between "{:?}" and "{} is that {:?} prints the byte as a number and {} prints the byte as a char, if reversed, it will print the byte as a string
                    println!("{:?} ({})\r", b, c);
                }

                // checks if the byte is q to exit the program
                if b == to_ctrl_byte('q') {
                    break;
                }
            }

            Err(err) => die(err),
        }
    }
}
