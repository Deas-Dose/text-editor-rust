// imports the necesssary crates
use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn die(e: std::io::Error) {
    // prints the error
    panic!("{}", e);
}

// main function
fn main() {
    // creates a raw terminal
    let _stdout = stdout().into_raw_mode().unwrap();
    
    for key in io::stdin().keys() {
        // match the key
        match key {
            // if the key is Ok, then match the key
            Ok(key) => match key {
                // prints the key and the character
                Key::Char(c) => {
                    if c.is_control() {
                        // if the key is a control key, then print the key
                        println!("{:?}\r", c as u8);
                    } else {
                        // if the key is not a control key, then print the key and the character
                        println!("{:?} ({})\r", c as u8, c);
                    }
                }

                // if the key is Ctrl + q, then break
                Key::Ctrl('q') => break,
                // for any unhandled key, then print the key
                _ => println!("{:?}\r", key),
            }

            Err(e) => die(e),
        }
    }
}

