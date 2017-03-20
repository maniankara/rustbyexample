macro_rules! test {
    ($left: expr; and $right: expr) => {
        println!("{:?} and {:?} = {:?}", stringify!($left),
                stringify!($right),
                $left && $right)
    };

    ($left: expr; or $right: expr) => {
        println!("{:?} or {:?} = {:?}", stringify!($left),
                stringify!($right),
                $left || $right)
    };
}

fn main() {

    test!(1 == 2; or 3 == 4);

    test!(1 == 2; and 3 == 4);
}