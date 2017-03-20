
fn check_dividing(divident: i32, divisor: i32) -> Option<i32> {
    if (divisor == 0) {
        None
    } else {
        Some(divident/divisor)
    }
}

fn divide(divident: i32, divisor: i32) {
    match check_dividing(divident, divisor) {
        None => println!("Division by zero"),
        Some(quotient) => println!("Quotient: {}", quotient)
    }
}

fn main() {

    divide(5,0);
    divide(5, 10);
}