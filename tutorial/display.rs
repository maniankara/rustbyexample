use std::fmt;

// this is an excerise to override a to_s or toString()

struct MinMax(i64, i64);

struct Point2D {
    x: f64,
    y: f64
}


impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}


impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X: {} and Y: {}", self.x, self.y)
    }
}



fn main() {

    let minmax = MinMax(10,5);

    println!("Comparing structures");

    println!("Display: {}", minmax);


    let points = Point2D{ x: 4.0, y: 6.0};

    println!("points: {}", points);

}