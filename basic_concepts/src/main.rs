mod mods;
mod dice;

use mods::{ip, coin, sub_mod};
use sub_mod::test;

use std::fs::File;
use std::io::{self, Read};

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    
    println!("Lets' go\n");
    let x = err_func();
    println!("{:?}", x);
}

fn err_func() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("error_file")?.read_to_string(&mut s)?;
    return Ok(s);
    // let f = File::open("error_file");
    // let mut f = match f {
    //     Ok(f) => f,
    //     Err(e12) => return Err(e12)
    // };

    // let mut s = String::new();
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e)
    // }
    // f.read_to_string(&mut s)?;
    // Ok(s)
}