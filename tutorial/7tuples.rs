use std::fmt;


struct Matrix(f64, f64, f64, f64);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "( {} {} )\n", self.0, self.1));
        write!(f, "( {} {} )", self.2, self.3)
    }
}

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}

fn transpose(matrix: Matrix) -> Matrix {
    Matrix (matrix.0, matrix.2, matrix.1, matrix.3)
}

fn main() {

    let matrix = Matrix (1.1, 1.2, 2.1, 2.2);

    println!("Matrix \n{}", matrix); // Activity1

    println!("Transpose \n{}", transpose(matrix)); // Activity2


}