#[derive(PartialEq, PartialOrd, Debug)]
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_cms(&self) -> Centimeters {
        let & Inches(inches) = self;
        Centimeters (inches as f64 * 2.54)
    }
}

fn main() {
    let inch = Inches(10);

    println!("{:?} inches is {:?} cms", inch, inch.to_cms());
    
}