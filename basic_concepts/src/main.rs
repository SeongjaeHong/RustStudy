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

use std::cmp::PartialOrd;

struct Point<X1, Y1> {
    x1: X1,
    y1: Y1
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, a>(self, other: Point<X2, a>) -> Point<X1, a> {
        Point {
            x1: self.x1,
            y1: other.y1
        }
    }
}

fn main() {
    // let x = err_func();
    // println!("{:?}", x);
    let p1: Point<i32, f64> = Point {x1:3,y1:5.2};
    let p2 = Point {x1: 'a', y1: "hello"};
    let p3 = p1.mixup(p2);
    println!("p3.x: {}, p3.y: {}", p3.x1, p3.y1);
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