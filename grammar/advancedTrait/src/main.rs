use std::ops::Add;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

fn main() {
    let p1 = Point{x:15, y:32};
    let p2 = Point{x:10, y:20};
    let p3 = p1 + p2;
    println!("add: {:?}", p3);
}
