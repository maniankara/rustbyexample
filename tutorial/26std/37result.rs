mod checked {

    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NegetiveLogrithm,
        NegetiveSquareRoot
    }

    pub type MathResult<f64> = Result<f64, MathError> ;

    pub fn div(d: f64, i: f64) -> MathResult<f64> {
        if i == 0f64  {
            Err(MathError::DivisionByZero)
        } else {
            Ok(d/i)
        }
    }

    pub fn sqrt(i: f64) -> MathResult<f64> {
        if i <= 0f64  {
            Err(MathError::NegetiveSquareRoot)
        } else {
            Ok(i.sqrt())
        }
    }

    pub fn ln(i: f64) -> MathResult<f64> {
        if i <= 0f64 {
            Err(MathError::NegetiveLogrithm)
        } else {
            Ok(i.ln())
        }
    }

}

fn op(d: f64, i: f64) -> f64{
    match checked::div(d, i) {
        Err(e) => panic!("Error: {:?}", e),
        Ok(q) => match checked::ln(q) {
            Err(e) => panic!("Error: {:?}", e),
            Ok(s) => match checked::sqrt(s) {
                Err(e) => panic!("Error: {:?}", e),
                Ok(r) => r
            }
        }
    }
}

fn op_try(d: f64, i: f64) -> checked::MathResult<f64> {
    let ratio = try!(checked::div(d,i));
    let ln = try!(checked::ln(ratio));
    checked::sqrt(ln)
}

fn op_(d: f64, i: f64) -> f64 {
    match op_try(d, i) {
        Ok(q) => q,
        Err(e) => panic!(e)
    }
}



fn main() {
    println!("operate: 50.0, 2.0: {}", op(50.0, 2.0));
    // println!("operate: 50.0, 2.0: {}", op(2.0, 50.0));

    println!("operate_try: 50.0, 2.0: {}", op_(50.0, 2.0));
    println!("operate: 50.0, 2.0: {}", op_(20.0, 50.0));
}



























