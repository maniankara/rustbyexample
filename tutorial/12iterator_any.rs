/*
pub trait Iterator {

    type Item;

    // `any` takes `&mut self` meaning the caller may be borrowed
    // and modified, but not consumed.
    fn any<F>(&mut self, f. F) -> bool where
        // `FnMut` meaning any captured variable may at most be
        // modified, not consumed. `Self::Item` states it takes
        // arguments to the closure by value.
        F: FnMut(Self::Item) -> bool {}


 }


*/

fn main() {

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 8, 9];

    println!("2 is present: {}", vec1.iter().any(|&x| x == 2));
    println!("2 is present: {}", vec2.into_iter().any(|x| x == 2));

    let arr1 = [1, 2, 3];
    let arr2 = [4, 8, 9];

    println!("2 is present: {}", arr2.iter().any(|&x| x == 2));
    println!("2 is present: {}", arr1.into_iter().any(|&x| x == 2)); // still yields &i32

    // tmp

    println!("first value: {:?}", vec1.get(0).map(|i| i).unwrap());
}