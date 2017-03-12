use std::fmt::Debug;

trait HasArea {
    fn  area(&self) -> f64;
}


#[derive(Debug)]
struct Rectangle { length: f64, height: f64}

#[allow(dead_code)]
struct Triangle { length: f64, height: f64}

impl HasArea for Rectangle {

    fn area(&self) -> f64 {
        self.length * self.height
    }
}

fn print_debug<T: Debug>(t: &T) { // allows only Ts implementing Debug
    println!("{:?}",t);
}


fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

fn main() {

    let rect = Rectangle { length: 5.0, height: 4.0};

    let tri = Triangle { length: 6.0, height: 7.0 };

    print_debug(&rect);
    println!("Area: {}", area(&rect));

    // fails
    // print_debug(&tri); // the trait `std::fmt::Debug` is not implemented for `Triangle`
    // println!("Area: {}", area(&tri));// the trait `HasArea` is not implemented for `Triangle`


}
