fn eat_box_i32(i: Box<i32>) {
    println!("box eaten: {}",i);
}

fn borrow_box_i32(i: &Box<i32>) {
    println!("borrowed: {}",i);
}

fn eat_i32(j: i32) {
    println!("eaten i32: {}", j);
}

fn borrow_i32(j: &i32) {
    println!("borrow: {}",j);
}

fn main() {

    // box
    let box_i32 = Box::new(5i32);
    borrow_box_i32(&box_i32);
    eat_box_i32(box_i32);
    // println!("Try to print box: {}", box_i32);

    let stack_i32 = 5i32;
    borrow_i32(&stack_i32);
    eat_i32(stack_i32);
    // println!("Try to print i32: ", stack_i32);

}