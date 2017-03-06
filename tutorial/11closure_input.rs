use std::mem;

fn apply<F>(f: F)
    where F: FnOnce() {
  f();
}


fn apply_2<F>(f: F) -> i32
    where F: Fn(i32) -> i32 {
    f(3)
}


fn main() {
    let greeting = "Hello";


    let mut farewell = "Goodbye".to_owned();

    let display = || {

        // Fn
        println!("{}", greeting);

        // FnMut
        farewell.push_str("!!!!");
        println!("{}", farewell);

        // FnOnce
        mem::drop(farewell)
    };


    let simple = |x| x+2;

    apply(display);

    println!("2 Added to 3 is: {}", apply_2(simple));

}