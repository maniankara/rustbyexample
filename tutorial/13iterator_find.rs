/*
pub trait Iterator {

    // `find` takes `&mut self` meaning the caller may be borrowed
    // and modified, but not consumed.
    fn find(&mut self, predicate: P) -> Option<Self::Item> where
        // `FnMut` meaning any captured variable may at most be
        // modified, not consumed. `&Self::Item` states it takes
        // arguments to the closure by reference.
        P: FnMut(&Self::Item) -> bool {}

}


*/


fn main() {

    let vec1 = vec![1,2,3];
    let vec2 = vec![4,8,9];

    println!("2 is present: {:?}", vec1.iter().find(|&&x| x==2));
    println!("2 is present: {:?}", vec2.into_iter().find(|&x| x==2));

    let arr1 = [1,2,3];
    let arr2 = [4,8,9];

    println!("2 is present: {:?}", arr1.into_iter().find(|&&x| x==2)); // still yields &&i32
    println!("2 is present: {:?}", arr2.iter().find(|&&x| x==2));

}