struct Point {
    x: i32,
    y: i32
}

struct Rectangle {
    p1: Point,
    p2: Point
}

fn rect_area(rect: Rectangle) {

    let Rectangle {p1: po1, p2: po2} = rect;
    let Point {x: a, y: b}  = po1;
    let Point {x: c, y: d} = po2;
    //calculate area with a,b,c,d
    println!("The points are: ({}, {}, {}, {})", a,b,c,d);
}

fn main() {
    let point = Point { x: 3, y: 6 };

    let Point { x: a, y: b } = point;

    println!("Points: ({}, {})", point.x, point.y);

    let point2 = Point { x: 20, y: 14};

    let rect = Rectangle { p1: point, p2: point2}; //Activity1
    rect_area(rect)

    // Activity2: TODO

}