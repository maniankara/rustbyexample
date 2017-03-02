use std::fmt;

// this is an excerise to override a to_s or toString()

struct MinMax(i64, i64);

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64

}

#[derive(Debug)]
struct Complex {
    real: f64,
    img: f64
}


impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}


impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "X: {} and Y: {}", self.x, self.y)
        write!(f, "{} + {}i", self.x, self.y) // activity

    }
}




fn main() {

    let minmax = MinMax(10,5);

    println!("Comparing structures");

    println!("Display: {}", minmax);


    let points = Point2D{ x: 4.0, y: 6.0};

    println!("Display points: {}", points);

    println!("Debug points: {:?}", points);


    let complex = Complex {real: 3.3, img: 7.2};
    println!("Debug: {:?}", complex);

}