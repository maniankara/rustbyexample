use std::thread;

static NTHREADS: i32 = 10;

fn main() {

    let mut children = vec![];

    for i in 0..NTHREADS {
        children.push(
            thread::spawn(move || {
                println!("Thread no: {}", i);
            })
        );
    }

    // waiter
    for c in children {
        let _ = c.join();
    }

}