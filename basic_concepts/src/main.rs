fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    

    let r3 = &mut s; // Can't refer to mutable at the same time with a refence to unmutable
    println!("changed s : {r3}");
    println!("x is {r1}");
}