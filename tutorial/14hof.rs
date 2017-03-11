// Find the sum of all the squared odd numbers under 1000"

fn is_odd(i: i32) -> bool {
    i % 2 != 0
}


fn main() {

    println!("Find the sum of all the squared odd numbers under 1000");

    let upper = 1000;

    // Higher Order Function approach

    let hof : i32 =
        (0..).map(|n|n*n)               // All natural numbers squared
        .take_while(|&n| n<upper)       // Below upper limit
        .filter(|&n| is_odd(n))         // That are odd
        .fold(0, |sum ,i|sum+i);        // Sum them

    println!("functional style: {}", hof);

}