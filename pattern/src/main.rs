
fn main() {
    let msg = 52;

    match msg {
        msg if msg==5 => println!("wow: {}", msg),
        _ => println!("ELSE: {}", msg),
    }
}