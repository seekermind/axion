use std::{char, io::{self, Read}};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Editor{}
    }
    pub fn run(&self){
        enable_raw_mode().unwrap();
        for i in io::stdin().bytes() {
            match i {
                Ok(i) => {
                    let c = i as char;
                    if c.is_control() {
                        println!("Binary: {0:08b} ASCII: {0:#03} \r", i);
                    } else {
                        println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}\r", i, c);
                    }
                    if c == 'q' {
                        break;
                    }
                }
                Err(err) => println!("Error: {}", err),
            }
        }
        disable_raw_mode().unwrap()
    }
}

