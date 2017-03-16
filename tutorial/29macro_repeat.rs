macro_rules! find_min {
    ($x:expr) => {$x};
    ($x:expr, $($y:expr),+ ) => {
        std::cmp::min($x, find_min!($($y),+))
    }
}

fn main() {
    println!("Min of 1: {}", find_min!(1u32));
    println!("Min of 1, 25: {}", find_min!(1u32, 25u32));
    println!("Min of 1, 3,5,0,2,4: {}", find_min!(1, 3,5,0,2,4));
}