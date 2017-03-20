use std::fmt;
use std::num::ParseIntError;

type Result<T> = std::result::Result<T, DoubleError>;

enum DoubleError {
    EmptyVec,

    Parse(ParseIntError)
}

impl From<ParseIntError> for DoubleError {
    fn from(err: ParseIntError) -> DoubleError {
        DoubleError::Parse(err)
    }
}

impl fmt::Display for DoubleError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DoubleError::EmptyVec => write!(f, "use vector with atleast one element!"),
            DoubleError::Parse(ref e) => e.fmt(f)
        }
    }
}


// double the first element with exception handling
fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        .ok_or(DoubleError::EmptyVec)
        .and_then(|s|s.parse::<i32>()
            .map_err(DoubleError::Parse)
            .map(|i| i *2))
}

fn double_first2(vec: Vec<&str>) -> Result<i32> {
    let first = try!(vec.first().ok_or(DoubleError::EmptyVec));
    let parsed = try!(first.parse::<i32>());

    Ok(2 * parsed)
}


fn print(r: Result<i32>) {
    match r {
        Ok(n) => println!("The first element double is: {}", n),
        Err(e) => println!("Error: {}", e)
    }
}

fn main() {

    let normal_vec = vec!["10", "12", "15"];
    let normal_vec2 = normal_vec.clone();

    let empty_vec = vec![];
    let empty_vec2 = empty_vec.clone();

    let failing_vec = vec!["tt", "et"];
    let failing_vec2 = failing_vec.clone();

    print(double_first(normal_vec));
    print(double_first(empty_vec));
    print(double_first(failing_vec));

    // with try!
    print(double_first2(normal_vec2));
    print(double_first2(empty_vec2));
    print(double_first2(failing_vec2));

}
