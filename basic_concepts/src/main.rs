mod mods;
mod dice;

use mods::{ip, coin};

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

enum Option<T> {
    None,
    Some(T),
    Anyval
}

fn main() {
    let rect1 = Rectangle::square(5);
    println!("area: {}", rect1.area());

    let home = ip::IpAddr::V4(String::from("127.0.0.1"));
    let company = ip::IpAddr::V6(String::from("I don't know"));

    println!("Cents: {}", coin::value_in_cents(coin::Coin::Nickel));

    let dice_roll = 3;
    if let 3 = dice_roll {
        dice::add_fancy_hat();
    }
    match dice_roll {
        3 => dice::add_fancy_hat(),
        7 => dice::remove_fancy_hat(),
        _ => (),
    }
}
