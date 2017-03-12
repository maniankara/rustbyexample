fn print_block(c: Box<i32>) {
    println!("value of c is: {}",c);
}


fn main() {
    let x = 10i32;
    let y = x;

    //Stack vars
    println!("Value of x is: {}" ,x);
    println!("Value of y is: {}", y);

    // Heap vars
    let a = Box::new(5i32);
    println!("Value of a is: {}",a);

    let b = a;
    // println!("Value of a is: {}",a); // value a used here after move

    print_block(b); // b is moved to print_block

    // println!("Value of a is: {}",b); // value b used here after move


    //************ Mutability Possibility *************//

    let a = Box::new(5i32);
    // a = Box::new(4); // compiler error

    println!("This by default is immutable box: {}",a);

    let mut b = a;

    b = Box::new(4);

    println!("This is a mutable box: {}",b);


}