use std::collections::HashSet;

fn main() {

    let mut set1: HashSet<i32> = vec!(1,2,3).into_iter().collect();

    let mut set2: HashSet<i32> = vec!(4,5,6).into_iter().collect();


    assert!(set1.insert(4));
    assert!(set1.contains(&4));

    println!("set1: {:?}", set1);
    println!("set2: {:?}", set2);

    // assert!(set1.insert(4));// panic

    println!("Union: {:?}", set1.union(&set2).collect::<Vec<&i32>>());
    println!("Intersection: {:?}", set1.intersection(&set2).collect::<Vec<&i32>>());
    println!("Difference: {:?}", set1.difference(&set2).collect::<Vec<&i32>>());
    println!("Symmetric diff: {:?}", set1.symmetric_difference(&set2).collect::<Vec<&i32>>());

}
