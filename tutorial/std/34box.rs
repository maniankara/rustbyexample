use std::mem;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64
}

struct Rectangle {
    p1: Point,
    p2: Point
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0}
}

fn boxed_origin() -> Box<Point> {
    Box::new(origin())
}

fn main() {
    let point = origin();

    let rectangle = Rectangle {
        p1: origin(),
        p2: Point { x: 3.0, y: 4.0 }
    };

    let boxed_point = Box::new(origin());

    let boxed_rectangle = Box::new( Rectangle {
        p1: origin(),
        p2: Point { x: 4.0, y: 5.0 }
    });

    println!("Point occupies: {}", mem::size_of_val(&point));
    println!("Boxed Point occupies: {}", mem::size_of_val(&boxed_point));

    println!("Rectangle occupies: {}", mem::size_of_val(&rectangle));
    println!("Boxed Rectangle occupies: {}", mem::size_of_val(&boxed_rectangle));

}