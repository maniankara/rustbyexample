fn main() {
    // simple
    println!("{} days", 31);
    println!(31); //passes

    // named
    println!("{count} days", count=31);

    // println!("My name is {0}, {1} {0}", "Bond");
    // width you can use only "0"
    println!("{number:0width$} with width", number=10, width=5);

    // print a struct
    #[allow(dead_code)]
    struct S(i32);
    // println!("struct: {}", S(5)); // throws error


    // Activity
    let pi = 3.141592;
    println!("{}", format!("{:.*}", 2, pi));


}