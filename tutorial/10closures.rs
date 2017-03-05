fn main() {
    // simple closure
    fn function (i: i32) -> i32 { i+1 }

    // annotated closures
    let annotated_closure = |i: i32| -> i32 { i +1 };

    // inferred closure
    let inferred_closure = | i | i+1 ; // OR |i| { i+1 }



    let i = 1;
    println!("simple: {}", function(i));
    println!("annotated: {}", annotated_closure(i));
    println!("inferred: {}", inferred_closure(i));

}