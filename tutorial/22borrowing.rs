struct Point {
    x: i32,
    y: i32,
    z: i32
}

fn main() {

    let mut point = Point {x: 1, y: 2, z: 0};

    //  - Immutable borrow as many times
    //  - Either mutable borrow or immutable borrow, not both
    {
        let immutable_borrow = &point;
        let second_immu_borrw = &point;

        // let mutable_borrow = &mut point; // fails: mutable borrow occurs here

    }

    // - Either mutable/immutale borrow, not both
    // - Cannot even reference the point var
    // - Can use the mutable borrow
    {
        let mut mutable_borrow = &mut point;
        mutable_borrow.z = 1;

        // let y = &point; // fails: immutable borrow occurs here

        // println!("z is: {}", point.z); // fails: immutable borrow occurs here
        println!("z is: {}", mutable_borrow.z);
    }

    println!("z is: {}", point.z);

    let immutable_borrow = &point;

}