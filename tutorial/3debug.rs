fn main() {

    #[derive(Debug)]
    struct Structure(i32);

    #[derive(Debug)]
    struct Nested(Structure);

    println!("{:?}", Structure(10)); // prints Structure(10)

    println!("{:?}", Nested(Structure(20))); // prints Nested(Structure(20))

    // because, these are not implemented in fmt::Display or fmt::Debug
}