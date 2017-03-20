use std::num::ParseIntError;

fn double_number(s: &str) -> Result<i32, ParseIntError> {
    match s.parse::<i32>() {
        Ok(n) => Ok(n * 2),
        Err(e) => Err(e)
    }
}

fn double_with_map(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>().map(|n| n*2)
}

type AliasedResult<T> = Result<T, ParseIntError>;

fn double_with_alias(s: &str) -> AliasedResult<i32> {
    s.parse::<i32>().map(|n| n*2)
}

fn main() {
    println!("Double 10: {:?}", double_number("10"));
    println!("Double tt: {:?}", double_number("tt"));

    println!("Double 20: {:?}", double_number("20"));

    println!("Double 20: {:?}", double_with_alias("20"));
}